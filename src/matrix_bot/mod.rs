mod commands;
mod business_logic;

pub mod matrx_bot {
    use std::io::Cursor;
    use matrix_sdk::{room::Room, Client, SyncSettings, ruma::{UserId,
                                                              events::{SyncMessageEvent,
                                                                       MessageEvent,
                                                                       AnyMessageEventContent,
                                                                       room::message::MessageEventContent, StrippedStateEvent,
                                                                       room::member::MemberEventContent
                                                              },
                                                              api::client::r0::room::get_room_event
    }, RoomMember};
    use matrix_sdk::ruma::events::room::message::{MessageFormat, MessageType, Relation, TextMessageEventContent};
    use crate::matrix_bot::matrx_bot::get_room_event::Request;

    use crate::{Config, DataLayer};
    use crate::lnbits_client::lnbits_client::LNBitsClient;
    use crate::matrix_bot::business_logic::BusinessLogicContext;
    use tokio::time::{sleep, Duration};
    use mime;
    use matrix_sdk::room::Joined;
    use matrix_sdk::ruma::{EventId, MilliSecondsSinceUnixEpoch, ServerName};
    use matrix_sdk::ruma::events::room::message::Relation::Reply;
    use simple_error::{bail, try_with};
    use simple_error::SimpleError;
    use crate::matrix_bot::commands::{balance, Command, donate, help, invoice, party, pay, send, tip, version};
    pub use crate::data_layer::data_layer::LNBitsId;

    #[derive(Debug)]
    struct ExtractedMessageBody {
        msg_body: Option<String>,
        formatted_msg_body: Option<String>
    }

    impl ExtractedMessageBody {
        fn new(msg_body: Option<String>,
               formatted_msg_body: Option<String>) -> ExtractedMessageBody {
            ExtractedMessageBody {
                msg_body,
                formatted_msg_body
            }
        }

        fn empty() ->  ExtractedMessageBody{
            ExtractedMessageBody::new(None, None)
        }
    }

    async fn auto_join(room_member: StrippedStateEvent<MemberEventContent>,
                       client: Client,
                       room: Room) {
        if room_member.state_key != client.user_id().await.unwrap() {
            return;
        }

        if let Room::Invited(ref room) = room {


            log::info!("Autojoining room {}", room.room_id());
            let mut delay = 2;

            while let Err(err) = room.accept_invitation().await {
                // retry autojoin due to synapse sending invites, before the
                // invited user can join for more information see
                // https://github.com/matrix-org/synapse/issues/4345
                log::error!("Failed to join room {} ({:?}), retrying in {}s", room.room_id(), err, delay);

                sleep(Duration::from_secs(delay)).await;
                delay *= 2;

                if delay > 3600 {
                    log::error!("Can't join room {} ({:?})", room.room_id(), err);
                    break;
                }
            }
            log::info!("Successfully joined room {}", room.room_id());
        }

        // Upon succesfull join send a single message
        let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
            "Thanks for inviting me. I support the following commands:\n".to_owned() +
                  BusinessLogicContext::get_help_content().as_str()
        ));

        let result = client.room_send(room.room_id(), content, None).await;
        match result {
            Err(error) => {
                log::warn!("Could not send welcome message due to {:?}..", error);
            }
            _ => { /* ignore */}
        }
    }

    fn reply_event_id(option: &Option<Relation>) -> Option<EventId> {
        if option.is_none() {  None }
        else {
            match option.as_ref().unwrap() {
                Reply { in_reply_to} => {
                    Some(in_reply_to.event_id.clone())
                },
                _ => {
                    None
                }
            }
        }
    }

    fn last_line<'a>(msg_body: &str) -> String {
        msg_body.split('\n').last().unwrap().to_string()
    }

    async fn extract_command(room: &Joined,
                             sender: &str,
                             event: &SyncMessageEvent<MessageEventContent>,
                             original_event: Option<EventId>,
                             extracted_msg_body: &ExtractedMessageBody) -> Result<Command, SimpleError> {
        let msg_body = extracted_msg_body.clone().msg_body.clone().unwrap().to_lowercase(); // We don't care about the case of the command.

        if last_line(msg_body.as_str()).starts_with("!tip") && !original_event.is_none() {
            let original_event = room.event(Request::new(&room.room_id(),
                                                         &original_event.unwrap())).await;
            match original_event {
                Ok(original_event_) => {
                    let answer = original_event_.event.deserialize();
                    let replyee: UserId = match answer {
                        Ok(any_room_event) => {
                            any_room_event.sender().clone()
                        }
                        _ => {
                            bail!("Could not parse answer {:?}", answer)
                        }
                    };
                    tip(sender,
                        event.event_id.as_str(),
                         last_line(msg_body.as_str()).as_str(),
                        replyee.as_str())
                },
                Err(simple_error) => {
                    log::error!("Error while retrieving original message {:?} ..", simple_error);
                    bail!("Could not retrieve original message {:?}", simple_error)
                }
            }
        }  else if msg_body.starts_with("!balance") {
            balance(sender, event.event_id.as_str())
        } else if msg_body.starts_with("!send") {
            let msg_body = preprocess_send_message(&extracted_msg_body, room).await;
            match msg_body {
                Ok(msg_body) => {
                    send(sender, event.event_id.as_str(), msg_body.as_str())
                },
                Err(_) => {
                    let error_message = "Sorry I did not recognize the username (or two or more users were possible candidates).\n \
                                              Please write out the user in full. I.e. like @username:example-server.com.";
                    let result = send_reply_to_event_in_room(&room,
                                                                               &event,
                                                                          error_message).await;
                    match result {
                        Err(error) => {
                            log::warn!("Could not send reply message due to {:?}..", error);
                        }
                        _ => { /* ignore */}
                    }
                    Ok(Command::None)
                }
            }
        } else if msg_body.starts_with("!invoice") {
            invoice(sender, event.event_id.as_str(), msg_body.as_str())
        } else if msg_body.starts_with("!pay") {
            pay(sender, event.event_id.as_str(), msg_body.as_str())
        } else if msg_body.starts_with("!help") {
            help(sender, event.event_id.as_str())
        } else if msg_body.starts_with("!donate") {
            donate(sender, event.event_id.as_str(), msg_body.as_str())
        } else if msg_body.starts_with("!party") {
            party(sender, event.event_id.as_str())
        } else if msg_body.starts_with("!version") {
            version(sender, event.event_id.as_str())
        } else {
            Ok(Command::None)
        }
    }

    // TODO(AE): Terrible code refactor
    async fn find_user_in_room(partial_user_id: &str,
                               room: &Joined) -> Result<Option<UserId>, SimpleError> {

        log::info!("Trying to find {:?} in room ..", partial_user_id);
        if partial_user_id.is_empty() { return Ok(None) }

        let split :Vec<&str> = partial_user_id.split(':').collect();
        if split.len() > 1 { return Ok(None) }

        let partial_user_id = split[0];

        if partial_user_id.is_empty()
            || ((partial_user_id.chars().next().unwrap() == '@') && partial_user_id.len() == 1) {
            return Ok(None)
        }

        let partial_user_id: String = if partial_user_id.chars().next().unwrap() == '@' { partial_user_id[1..].to_string() }
                                      else { partial_user_id.to_string() };

        let mut matched_user_id: Option<UserId> = None;

        let members: Vec<RoomMember> = try_with!(room.members_no_sync().await,
                                                 "Could not get room members");
        for member in members {
            log::info!("comparing {:?} & {:?} vs {:?}",
                       member.user_id(),
                       member.user_id().localpart(),
                       partial_user_id);
            if member.user_id().localpart() == partial_user_id {
                if matched_user_id.is_none() {
                    matched_user_id = Some(member.user_id().clone())
                } else {
                    log::info!("Found multiple possible matching user names, not returning anything");
                    return Ok(None)
                }
            }
        }

        Ok(matched_user_id)
    }

    fn try_to_parse_into_full_username(username: &str) -> Option<UserId> {
        log::info!("Trying to parse {:?} into a full username ..", username);
        let split: Vec<&str> = username.split(':').collect();
        if split.len() != 2 {
            return  None
        }

        let server_name = <&ServerName>::try_from(split[1]);
        if server_name.is_err() {
            return None
        }
        let server_name = server_name.unwrap();

        let user_id = UserId::parse_with_server_name(username, server_name);

        match user_id {
            Ok(user_id) => { Some(user_id) }
            _ => None
        }
    }

    async fn preprocess_send_message(extracted_msg_body: &ExtractedMessageBody,
                                     room: &Joined) -> Result<String, SimpleError> {

        log::info!("Preprocessing {:?} for send ..", extracted_msg_body);

        let raw_message = extracted_msg_body.msg_body.clone().unwrap();
        let split_message : Vec<&str> = raw_message.split_whitespace().collect();

        if split_message.len() < 3 {
            bail!("Not a valid send message")
        }

        let mut target_id: Option<UserId> = try_to_parse_into_full_username(split_message[2]);
        target_id = if target_id.is_some() { target_id }
                    else {
                        try_with!(find_user_in_room(split_message[2], room).await,
                                      "Error while trying to find user")
                    };
        target_id = if target_id.is_some() { target_id }
                    else {
                        if extracted_msg_body.formatted_msg_body.is_none() {
                            None
                        } else {
                            let s = extracted_msg_body.formatted_msg_body.clone().unwrap();
                            extract_user_from_formatted_msg_body(s.as_str())
                        }
                    };

        if target_id.is_none() {
            bail!("Could not preprocess message with a valid id")
        }

        let target_id = target_id.unwrap();

        let new_message_parts = [&[split_message[0]],
                                 &[split_message[1]],
                                 &[target_id.as_str()],
                                 &split_message[3..]].concat();

        let preprocessed_message = new_message_parts.join(" ").to_string();

        log::info!("Created the following message {:?} for send ..", preprocessed_message);

        Ok(preprocessed_message)
    }

    fn extract_user_from_formatted_msg_body(formatted_msg_body: &str) -> Option<UserId> {

        let dom = tl::parse(formatted_msg_body,tl::ParserOptions::default());
        let mut img = dom.query_selector("a[href]").unwrap();
        let img = img.next();
        if img.is_none() {
            return None
        }

        let parser = dom.parser();
        let a = img.unwrap().get(parser);
        if a.is_none() {
            return None
        }

        // We know this exists because of the above statements
        let inner_text = a.unwrap()
                                 .as_tag()
                                 .unwrap()
                                 .attributes()
                                 .get_attribute("href")
                                 .unwrap()
                                 .unwrap()
                                 .as_utf8_str()
                                 .to_string();

        let r: Vec<&str> = inner_text.split('@').collect();
        if r.len() != 2 { return None }

        let complete_id = ("@".to_owned() + r[1]).to_string();

        let user_id = UserId::try_from(complete_id);

        if user_id.is_ok() {
            Some(user_id.unwrap())
        } else {
            None
        }
    }

    fn extract_body(event: &SyncMessageEvent<MessageEventContent>) -> ExtractedMessageBody {
        if let SyncMessageEvent {
            content:
            MessageEventContent {
                msgtype: MessageType::Text(TextMessageEventContent { body: msg_body,
                                                                     formatted,
                                                                     .. }),
                ..
            },
            ..
        } = event
        {
            let formatted_message_body: Option<String> = if formatted.is_some() {
                let unwrapped = formatted.clone().unwrap();
                match unwrapped.format {
                    MessageFormat::Html => { Some(unwrapped.body) } // We only support html for now
                    _ => { None }
                }
            } else {
                None
            };

            ExtractedMessageBody::new(Some(msg_body.clone()), formatted_message_body)
        } else {
            log::warn!("could not parse body..");
            ExtractedMessageBody::empty()
        }
    }

    async fn send_reply_to_event_in_room(room: &Joined,
                                         event: &SyncMessageEvent<MessageEventContent>,
                                         reply: &str) -> Result<(), SimpleError> {
        let message_event = MessageEvent {
            content: event.content.clone(),
            event_id: event.event_id.clone(),
            sender: event.sender.clone(),
            origin_server_ts: event.origin_server_ts,
            room_id: room.room_id().clone(),
            unsigned: event.unsigned.clone()
        };

        let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_reply_plain(
            reply,
            &message_event
        ));

        log::info!("Replying with content {:?} ..", content);
        try_with!(room.send(content, None).await, "Could not send message");
        Ok(())
    }

    pub struct MatrixBot {
        client: Client,
        business_logic_contex: BusinessLogicContext,
        config: Config
    }

    impl MatrixBot {

        pub async fn new(data_layer: DataLayer,
                         lnbits_client: LNBitsClient,
                         config: &Config ) -> matrix_sdk::Result<MatrixBot> {

            let user_id = UserId::try_from(config.matrix_server.clone())?;

            log::info!("Creating client ..");

            let client = Client::new_from_user_id(user_id.clone()).await?;

            log::info!("Loging client in ..");

            client.login(user_id.localpart(),
                                    config.matrix_password.as_str(),
                                    None,
                                    None).await?;

            let matrix_bot = MatrixBot {
                business_logic_contex: BusinessLogicContext::new(lnbits_client,
                                                                 data_layer,
                                                                 config),
                client,
                config: config.clone()
            };

            log::info!("Done with preliminary steps ..");

            Ok(matrix_bot)
        }

        pub async fn init(&self) {

            log::info!("Performing init ..");

            // Dangerous

            self.client.register_event_handler(auto_join).await;

            let business_logic_contex = self.business_logic_contex.clone();
            let bot_name = self.bot_name().clone();
            let current_time = MilliSecondsSinceUnixEpoch::now();

            self.client.register_event_handler({
                let business_logic_contex = business_logic_contex.clone();
                let bot_name = bot_name.clone();
                let current_time = current_time.clone();
                move |event: SyncMessageEvent<MessageEventContent>, room: Room|{
                    let business_logic_contex = business_logic_contex.clone();
                    let bot_name = bot_name.clone();
                    async move {
                        if let Room::Joined(room) = room {
                            log::info!("processing event {:?} ..", event);

                            let sender = event.sender.as_str();
                            let original_event = reply_event_id(&(event.content.relates_to));
                            let extracted_msg_body = extract_body(&event);
                            if extracted_msg_body.msg_body.is_none() { return } // No body to process

                            if current_time > event.origin_server_ts {
                                // Event was before I joined, can happen in public rooms.
                                return;
                            }

                            let plain_message_body = extracted_msg_body.msg_body.clone().unwrap();

                            if plain_message_body.starts_with(bot_name.as_str()) {
                                let result = send_reply_to_event_in_room(&room,
                                                                                           &event,
                                                                                     "Thanks for you message. I am but a simple bot. I will join any room you invite me to. Please run !help to see what I can do.").await;
                                match result {
                                    Err(error) => {
                                        log::warn!("Could not send reply message due to {:?}..", error);
                                    }
                                    _ => { /* ignore */}
                                }
                                return
                            }

                            let command = extract_command(&room,
                                                                                  sender,
                                                                                  &event,
                                                                                  original_event,
                                                                                  &extracted_msg_body).await;


                            match command {
                                Err(error) => {
                                    log::warn!("Error occurred while extracting command {:?}..", error);
                                    let result = send_reply_to_event_in_room(&room,
                                                                                               &event,
                                                                                          "I did not understand that command, please use '!help' to list the commands and how to use them").await;
                                    match result {
                                        Err(error) => {
                                            log::warn!("Could not even send error message due to {:?}..", error);
                                        }
                                        _ => { /* ignore */}
                                    }
                                    return
                                }
                                _ => { },
                            };
                            let command = command.unwrap();
                            if command.is_none() { return } // No Command to execute

                            let command_reply = business_logic_contex.processing_command(command).await;
                            match command_reply {
                                Err(error) => {
                                    log::warn!("Error occurred during business processing {:?}..", error);
                                    let result = send_reply_to_event_in_room(&room,
                                                                             &event,
                                                                             "I seem to be experiencing a problem please try again later").await;
                                    match result {
                                        Err(error) => {
                                            log::warn!("Could not even send error message due to {:?}..", error);
                                        }
                                        _ => { /* ignore */}
                                    }
                                    return
                                }
                                _ => { },
                            };
                            let command_reply = command_reply.unwrap();

                            log::info!("Sending back answer {:?}", command_reply);

                            if command_reply.is_empty() {
                                return // No output to give back
                            }

                            let send_result = send_reply_to_event_in_room(&room,
                                                                              &event,
                                                                         command_reply.text.unwrap().as_str()).await;
                            match send_result {
                                Err(error) => {
                                    log::warn!("Error occurred while sending response {:?}..", error);
                                    return
                                }
                                _ => { },
                            };

                            //
                            // TODO(AE) This assumes we don't have image only responses fix once
                            // this changes.
                            //

                            // Attaching image to message
                            if command_reply.image.is_some() {
                                // https://stackoverflow.com/questions/42240663/how-to-read-stdioread-from-a-vec-or-slice
                                let mut image_to_upload = Cursor::new(command_reply.image.unwrap());
                                let upload_result = room.send_attachment("image",
                                                                             &mime::IMAGE_PNG,
                                                                                 &mut image_to_upload,
                                                                                 None).await;
                                match upload_result {
                                    Err(error) => {
                                        log::warn!("Error occurred while attaching image {:?}..", error);
                                        return
                                    }
                                    _ => { },
                                }
                            }
                        }
                    }
                }
            }).await;
        }

        fn bot_name(&self) -> String {
            let parts: Vec<&str> = self.config.matrix_server.split(':').collect();
            if parts.is_empty() || parts[0].len() < 1 {
                log::warn!("Could not parse my own name from config, please check it");
                "".to_string()
            } else {
                parts[0][1..].to_owned()
            }
        }

        pub async fn sync(&self) -> matrix_sdk::Result<()>  {
            log::info!("Starting sync ..");
            Ok(self.client.sync(SyncSettings::default()).await)
        }
    }
}
