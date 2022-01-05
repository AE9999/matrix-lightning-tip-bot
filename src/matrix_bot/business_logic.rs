use chrono::Utc;
use simple_error::{bail, SimpleError, try_with};
use uuid::Uuid;
use qrcode_generator::QrCodeEcc;
use crate::{Config, DataLayer, LNBitsClient};
use crate::data_layer::data_layer::NewMatrixId2LNBitsId;
use crate::lnbits_client::lnbits_client::{CreateUserArgs, InvoiceParams, LNBitsUser, PaymentParams, Wallet, WalletInfo};
use crate::matrix_bot::commands::{Command, CommandReply};
use crate::matrix_bot::matrix_bot::LNBitsId;

#[derive(Clone)]
pub struct BusinessLogicContext  {
    lnbits_client: LNBitsClient,
    data_layer: DataLayer,
    config: Config
}

impl BusinessLogicContext {

    pub fn new(lnbits_client: LNBitsClient,
               data_layer: DataLayer,
               config: &Config) -> BusinessLogicContext {
        BusinessLogicContext {
            lnbits_client,
            data_layer,
            config: config.clone()
        }
    }

    pub fn get_help_content() -> String {
         format!("Matrix-Lightning-Tip-Bot {:?}  \n \
                 !tip      - Reply to a message to tip it: !tip <amount> [<memo>]\n\
                 !balance - Check your balance: !balance\n\
                 !send    - Send funds to a user: !send <amount> <@user> or <@user:domain.com> [<memo>]\n\
                 !invoice - Receive over Lightning: !invoice <amount> [<memo>]\n\
                 !pay     - Pay  over Lightning: !pay <invoice>\n\
                 !help    - Read this help.\n\
                 !donate  - Donate to the matrix-lighting-tip-bot project: !donate <amount>\n\
                 !party   - Start a Party: !party\n\
                 !version - Print the version of this bot\n", env!("CARGO_PKG_VERSION"))
    }

    pub async fn processing_command(&self,
                                command: Command) -> Result<CommandReply, SimpleError> {
        let command_reply = match command {
            Command::Tip { sender, event: _, amount, memo, replyee } => {
                try_with!(self.do_process_send(sender.as_str(),
                                               replyee.as_str(),
                                               amount,
                                               &memo).await,
                                               "Could not process tip.")
            },
            Command::Send { sender, event: _, amount, recipient, memo } => {
                try_with!(self.do_process_send(sender.as_str(),
                                               recipient.as_str(),
                                               amount,
                                               &memo).await,
                          "Could not process send.")
            },
            Command::Invoice { sender, event: _, amount, memo } => {
                try_with!(self.do_process_invoice(sender.as_str(),
                                                  amount,
                                                  &memo).await,
                          "Could not process invoice")
            },
            Command::Balance { sender, event: _ } => {
                try_with!(self.do_process_balance(sender.as_str()).await,
                                                  "Could not process balance")
            },
            Command::Pay { sender, event: _, invoice } => {
                try_with!(self.do_process_pay(sender.as_str(), invoice.as_str()).await,
                          "Could not process pay")
            },
            Command::Help { sender: _, event: _ } => {
                try_with!(self.do_process_help().await,
                          "Could not process help")
            },
            Command::Donate { sender, event: _, amount } => {
                try_with!(self.do_process_donate(sender.as_str(), amount).await,
                         "Could not process donate")
            }
            Command::Party { sender: _, event: _  } => {
                try_with!(self.do_process_party().await, "Could not process party")
            },
            Command::Version { sender: _, event: _  } => {
                try_with!(self.do_process_version().await, "Could not process party")
            },
            _ => {
                log::error!("Encountered unsuported command {:?} ..", command);
                bail!("Could not process: {:?}", command)
            }
        };
        Ok(command_reply)
    }

    async fn do_process_send(&self,
                             sender: &str,
                             recipient: &str,
                             amount: u64,
                             memo: &Option<String>) -> Result<CommandReply, SimpleError>  {
        log::info!("processing send command ..");


        let bolt11_invoice: String = try_with!(self.generate_bolt11_invoice_for_matrix_id(recipient, amount, memo).await,
                                        "Could not generate invoice");

        try_with!(self.pay_bolt11_invoice_as_matrix_is(sender, bolt11_invoice.as_str()).await,
                  "Could not pay invoice");

        if memo.is_some() {
            Ok(CommandReply::text_only(format!("{:?} send {:?} to {:?} with memo {:?}",
                                                    sender,
                                                    amount,
                                                    recipient,
                                                    memo.clone().unwrap()).as_str()))
        }
        else {
            Ok(CommandReply::text_only(format!("{:?} send {:?} to {:?}",
                                               sender,
                                               amount,
                                               recipient).as_str()))
        }

    }


    async fn do_process_invoice(&self,
                                sender: &str,
                                amount: u64,
                                memo: &Option<String>) -> Result<CommandReply, SimpleError> {
        log::info!("processing invoice command ..");

        let bolt11_invoice: String = try_with!(self.generate_bolt11_invoice_for_matrix_id(sender, amount, memo).await,
                                        "Could not generate invoice");

        log::info!("Generated {:?} as invoice", bolt11_invoice);

        let image: Vec<u8> = try_with!(qrcode_generator::to_png_to_vec(bolt11_invoice.as_str(),
                                                              QrCodeEcc::Medium,
                                                             256),
                                       "Could not generate QR code");

        // Insert QR code here
        let command_reply = CommandReply::new(bolt11_invoice.as_str(),
                                                                image);

        Ok(command_reply)
    }

    async fn do_process_balance(&self, sender: &str) -> Result<CommandReply, SimpleError> {
        log::info!("processing balance command ..");
        let lnbits_id = try_with!(self.matrix_id2lnbits_id(sender).await,
                                  "Could not load client");
        let wallet = try_with!(self.lnbits_id2wallet(&lnbits_id).await,
                                      "Could not load wallet");

        let wallet_info = try_with!(self.wallet2wallet_info(&wallet).await,
                                    "Could not load wallet info");

        let balance = wallet_info.balance;
        let balance = if balance.is_none()  { 0 }
                      else { balance.unwrap() / 1000  }; // Minisatashis are a bitch.

        Ok(CommandReply::text_only(format!("Your balance is {:?}", balance).as_str()))
    }

    async fn do_process_pay(&self, sender: &str, bol11_invoice: &str) -> Result<CommandReply, SimpleError> {
        log::info!("processing pay command ..");

        try_with!(self.pay_bolt11_invoice_as_matrix_is(sender, bol11_invoice).await,
                  "Could not pay invoice");

        Ok(CommandReply::text_only(format!("{:?} payed an invoice", sender).as_str()))
    }

    async fn do_process_help(&self) -> Result<CommandReply, SimpleError> {
        log::info!("processing help command ..");
        Ok(CommandReply::text_only(BusinessLogicContext::get_help_content().as_str()))
    }

    async fn do_process_party(&self) -> Result<CommandReply, SimpleError> {
        log::info!("processing party command ..");
        Ok(CommandReply::text_only("🎉🎊🥳 let's PARTY!! 🥳🎊🎉"))
    }

    async fn do_process_version(&self) -> Result<CommandReply, SimpleError> {
        Ok(CommandReply::text_only(format!("My version is {:?}", env!("CARGO_PKG_VERSION")).as_str()))
    }

    async fn do_process_donate(&self, sender: &str,  amount: u64) -> Result<CommandReply, SimpleError> {
        let result = self.do_process_send(sender,
                                                              self.config.donate_user.as_str(),
                                                                      amount,
                                                                &Some(format!("a generouse donation from {:?}", sender))).await;
        match result {
            Ok(_) => Ok(CommandReply::text_only("Thanks for the donation")),
            Err(error) => Err(error)
        }

    }

    async fn matrix_id2lnbits_id(&self, matrix_id: &str) -> Result<LNBitsId, SimpleError> {
        if !(self.data_layer.lnbits_id_exists_for_matrix_id(matrix_id)) {

            let wallet_name = matrix_id.clone().to_owned() + "wallet";
            let admin_id = Uuid::new_v4().to_string();
            let user_name = matrix_id.clone();
            let email = "";
            let password = "";

            let create_user_args = CreateUserArgs::new(wallet_name.as_str(),
                                                       admin_id.as_str(),
                                                       user_name,
                                                       email,
                                                       password);
            let result = self.lnbits_client.create_user_with_initial_wallet(&create_user_args).await;
            match  result {
                Ok(result) => {
                    log::info!("created {:?} ..", result);
                    let date_created = Utc::now().to_string();
                    let new_matrix_id_2_lnbits_id = NewMatrixId2LNBitsId::new(matrix_id,
                                                                              result.id.as_str(),
                                                                              result.admin.as_str(),
                                                                              date_created.as_str());
                    self.data_layer.insert_matrix_id_2_lnbits_id(new_matrix_id_2_lnbits_id);
                },
                Err(e) => {
                    // Ask how this stuff works
                    bail!("{:?}", e);
                }
            }
        }
        Ok(self.data_layer.lnbits_id_for_matrix_id(matrix_id))
    }

    async fn lnbits_id2wallet(&self, lnbits_id: &LNBitsId) -> Result<Wallet, SimpleError> {
        let lnbits_user = LNBitsUser::from_id(lnbits_id.lnbits_id.as_str());
        let mut wallets = try_with!(self.lnbits_client.wallets(&lnbits_user).await,
                                           "Could not retrieve wallets");
        if wallets.len() != 1 {
            bail!("Expected a single wallet got {:?}", wallets)
        }
        let wallet = wallets.remove(0);

        Ok(wallet)
    }

    async fn wallet2wallet_info(&self, wallet: &Wallet) -> Result<WalletInfo, SimpleError> {
        Ok(try_with!(self.lnbits_client.wallet_info(&wallet).await,
                     "Could not retrieve wallet"))
    }

    async fn pay_bolt11_invoice_as_matrix_is(&self,
                                             matrix_id: &str,
                                             bolt11_invoice: &str) -> Result<(), SimpleError> {
        let parsed_invoice: lightning_invoice::Invoice = try_with!(str::parse::<lightning_invoice::Invoice>(bolt11_invoice),
                                                            "could not parse invoice");

        if parsed_invoice.amount_milli_satoshis().is_none() {
            bail!( "Incorrect invoice")
        }
        let invoice_milli_satoshi_amount = parsed_invoice.amount_milli_satoshis().unwrap();

        log::info!("Got an amount for {:?} satoshis ..", invoice_milli_satoshi_amount / 1000);

        let payment_params = PaymentParams::new(true, bolt11_invoice);

        let lnbits_id = try_with!(self.matrix_id2lnbits_id(matrix_id).await,
                                          "Could not get lnbits id");

        let wallet = try_with!(self.lnbits_id2wallet(&lnbits_id).await,
                                      "Could not get wallet");

        try_with!(self.lnbits_client.pay(&wallet, &payment_params).await,
                  "Could not perform payment");

        Ok(())
    }

    async fn generate_bolt11_invoice_for_matrix_id(&self,
                                                   matrix_id: &str,
                                                   amount: u64,
                                                   memo: &Option<String>) -> Result<String, SimpleError> {

        let lnbits_id = try_with!(self.matrix_id2lnbits_id(matrix_id).await,
                                  "Could not load client");
        let wallet = try_with!(self.lnbits_id2wallet(&lnbits_id).await,
                                      "Could not load wallet");
        let invoice_params = InvoiceParams::simple_new(amount, memo);

        let invoice = try_with!(self.lnbits_client.invoice(&wallet, &invoice_params).await,
                                   "Could not load invoice");

        Ok(invoice.payment_request)
    }
}
