/*
 * AE: Implementation of API described at https://legend.lnbits.com/docs
 */

pub mod lnbits_client {
    use std::time::Duration;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use crate::Config;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct InvoiceParams {
        pub out: bool,
        pub amount: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memo: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub webhook: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description_hash: Option<String>
    }

    impl InvoiceParams {
        pub fn simple_new(amount: u64, memo: &Option<String>) -> InvoiceParams {
            InvoiceParams {
                out: false,
                amount: amount as i64,
                memo: if memo.is_none() { Some(Uuid::new_v4().to_string()) } else { memo.clone() },
                webhook: None,
                description_hash: None, // TODO(AE): I've no idea why this should work?
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
        pub struct PaymentParams {
        pub out: bool,
        pub bolt11: String,
    }

    impl PaymentParams {
        pub fn new(out: bool, bolt11: &str) -> PaymentParams {
            PaymentParams {
                out,
                bolt11: bolt11.to_string()
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TransferParams {
        pub memo: String,
        pub num_satoshis: i64,
        pub dest_wallet_id: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct BitInvoice {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub payment_hash: Option<String>,
        pub payment_request: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Wallet {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub admin: Option<String>,
        #[serde(rename = "adminkey")]
        pub admin_key: String,
        #[serde(rename = "inkey")]
        pub in_key: String,
        pub name: String,
        pub user: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub balance: Option<u64>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct WalletInfo {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub balance: Option<u64>,
    }



    #[derive(Debug, Deserialize, Serialize)]
    pub struct Error {
        pub name: String,
        pub message: String,
        pub code: String,
        pub status: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct LNBitsUser {
        pub id: String,
        pub name: String,
        // Primary key
        pub email: String,
        pub admin: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub wallets: Option<Vec<Wallet>>
    }

    impl LNBitsUser {
        pub fn from_id(lnbits_id: &str) -> LNBitsUser {
            // TODO(AE): Is there a differnce between id & admin id?
            LNBitsUser {
                id: lnbits_id.to_string(),
                name: "".to_string(),
                email: "".to_string(),
                admin: lnbits_id.to_string(),
                password: None,
                wallets: None,
            }
        }
    }

    #[derive(Clone)]
    pub struct LNBitsClient {
        pub url: String,
        pub header: [(String, String); 3],
    }

    impl LNBitsClient {

        pub fn new(config: &Config) -> LNBitsClient {
            return LNBitsClient {
                url: String::from(config.lnbits_url.clone()),
                header: [(String::from("Content-Type"), String::from("application/json")),
                    (String::from("Accept"), String::from("application/json")),
                    (String::from("X-Api-Key"), String::from(config.lnbits_x_api_key.clone()))]
            }
        }

        pub async fn create_user_with_initial_wallet(&self,
                                                     create_user_args: &CreateUserArgs) -> Result<LNBitsUser, reqwest::Error> {

            let response = reqwest::Client::new().post([self.url.as_str(), "/usermanager/api/v1/users"].join(""))
                .header((self.header[0]).0.as_str(), (self.header[0]).1.as_str())
                .header((self.header[1]).0.as_str(), (self.header[1]).1.as_str())
                .header((self.header[2]).0.as_str(), (self.header[2]).1.as_str())
                .json(create_user_args)
                .send()
                .await?
                .json::<LNBitsUser>()
                .await?;
            Ok(response)
        }

        pub async fn wallet_info(&self, wallet: &Wallet) -> Result<WalletInfo, reqwest::Error> {
            let header: [(String, String); 3] =
                [(String::from("Content-Type"), String::from("application/json")),
                    (String::from("Accept"), String::from("application/json")),
                    (String::from("X-Api-Key"), String::from(&wallet.in_key))];

            let response = reqwest::Client::new().get([self.url.as_str(), "/api/v1/wallet"].join(""))
                .header((header[0]).0.as_str(), (header[0]).1.as_str())
                .header((header[1]).0.as_str(), (header[1]).1.as_str())
                .header((header[2]).0.as_str(), (header[2]).1.as_str())
                .send()
                .await?
                /*.json::<WalletInfo>()
                .await?*/;

            log::info!("Received: {:?}", response);

            let response_text = response.text().await?;

            log::info!("Received Txt: {:?}", response_text);

            let response: WalletInfo = serde_json::from_str(response_text.as_str()).unwrap();

            Ok(response)
        }

        pub async fn wallets(&self, user: &LNBitsUser) -> Result<Vec<Wallet>, reqwest::Error> {
            let response = reqwest::Client::new().get([self.url.as_str(), "/usermanager/api/v1/wallets/", &*(user.id)].join(""))
                .header((self.header[0]).0.as_str(), (self.header[0]).1.as_str())
                .header((self.header[1]).0.as_str(), (self.header[1]).1.as_str())
                .header((self.header[2]).0.as_str(), (self.header[2]).1.as_str())
                .send()
                .await?
                /*.json::<Vec<Wallet>>()
                .await?*/;

            log::info!("Received: {:?}", response);

            let response_text = response.text().await?;

            log::info!("Received Txt: {:?}", response_text);

            let response: Vec<Wallet> = serde_json::from_str(response_text.as_str()).unwrap();

            Ok(response)
        }

        pub async fn invoice(&self,
                             wallet: &Wallet,
                             invoice_params: &InvoiceParams) -> Result<BitInvoice, reqwest::Error> {
            let response = reqwest::Client::new().post([self.url.as_str(), "/api/v1/payments"].join(""))
                .header((self.header[0]).0.as_str(), (self.header[0]).1.as_str())
                .header((self.header[1]).0.as_str(), (self.header[1]).1.as_str())
                .header((self.header[2]).0.as_str(), wallet.in_key.clone())
                .json(&invoice_params)
                .send()
                .await?
                .json::<BitInvoice>()
                .await?;

            Ok(response)
        }

        // AE: Funny how the telegram bot tries to put the answer of this into a BitInvoice, I wouldn't
        pub async fn pay(&self,
                         wallet: &Wallet,
                         payment_params: &PaymentParams) -> Result<(), reqwest::Error> {
            reqwest::Client::new().post([self.url.as_str(), "/api/v1/payments"].join(""))
                .header((self.header[0]).0.as_str(), (self.header[0]).1.as_str())
                .header((self.header[1]).0.as_str(), (self.header[1]).1.as_str())
                .header((self.header[2]).0.as_str(), wallet.admin_key.clone())
                .timeout(Duration::from_secs(3600))
                .json(&payment_params)
                .send()
                .await?
                .text()
                .await?;
            Ok(())
        }
    }


    #[derive(Debug, Deserialize, Serialize)]
    pub struct CreateUserArgs {
        pub wallet_name: String,
        pub admin_id: String,
        pub user_name: String,
        pub email: String,
        pub password: String,
    }

    impl CreateUserArgs {
        pub fn new(wallet_name: &str,
               admin_id: &str,
               user_name: &str,
               email: &str,
               password: &str) -> CreateUserArgs {
            CreateUserArgs {
                wallet_name: String::from(wallet_name),
                admin_id: String::from(admin_id),
                user_name: String::from(user_name),
                email: String::from(email),
                password: String::from(password)
            }
        }
    }
}




