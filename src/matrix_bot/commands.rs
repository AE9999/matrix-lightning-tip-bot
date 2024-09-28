use simple_error::{bail, SimpleError, try_with};

#[derive(Debug)]
pub enum Command  {
    Tip     { sender: String, amount: u64, memo: Option<String>, replyee: String },
    Balance { sender: String },
    Send    { sender: String, amount: u64, recipient: String, memo: Option<String> },
    Invoice { sender: String, amount: u64, memo: Option<String> },
    Pay     { sender: String, invoice: String },
    Help    {  },
    Donate  { sender: String, amount: u64 },
    Party   { },
    Version { },
    None,
}

#[derive(Debug)]
pub struct CommandReply {
    pub text: Option<String>,
    pub image: Option<Vec<u8>>
}

impl Command {

    pub fn is_none(&self) -> bool {
        match self {
            Command::None => true,
            _ => false
        }
    }
}

pub fn tip(sender:&str, text: &str, replyee: &str) -> Result<Command, SimpleError> {
    let split = text.split_whitespace().collect::<Vec<&str>>();
    if split.len() < 2 {
        bail!("Expected a at least 2 arguments")
    }
    let amount =   try_with!(split[1].parse::<u64>(), "could not parse value");
    let memo = if split.len() > 2 { Some(split[2..].join(" ") )  }
                            else { None };
    Ok(Command::Tip { sender: sender.to_string(),
                      replyee: replyee.to_string(),
                      amount,
                      memo })
}

pub fn balance(sender:&str)  -> Result<Command, SimpleError> {
    Ok(Command::Balance { sender: String::from(sender) } )
}

pub fn send(sender:&str,
            text: &str) -> Result<Command, SimpleError> {
    let split = text.split_whitespace().collect::<Vec<&str>>();

    if split.len() < 2 {
        bail!("Expected a at least 2 arguments")
    }
    let amount =  try_with!(split[1].parse::<u64>(), "could not parse value");
    let recipient = String::from(split[2]);
    let memo = if split.len() > 3 { Some(split[3..].join(" ") )  }
    else { None };
    Ok(Command::Send {  sender:String::from(sender),
                        amount,
                        recipient,
                        memo })
}

pub fn invoice(sender:&str,
               text: &str) -> Result<Command, SimpleError> {
    let split = text.split_whitespace().collect::<Vec<&str>>();
    if split.len() < 2 {
        bail!("Expected a at least 2 arguments")
    }
    let amount =  try_with!(split[1].parse::<u64>(), "could not parse value");
    let memo = if split.len() > 2 { Some(split[2..].join(" ") )  }
                            else { None };
    Ok(Command::Invoice { sender: String::from(sender), amount, memo })
}

pub fn pay(sender:&str,
           text: &str) -> Result<Command, SimpleError> {
    let split = text.split_whitespace().collect::<Vec<&str>>();
    if split.len() < 2 {
        bail!("Expected a at least 2 arguments")
    }
    let invoice = String::from(split[1]);
    Ok(Command::Pay { sender: String::from(sender),
                      invoice })
}

pub fn help() -> Result<Command, SimpleError> {
    Ok(Command::Help { })
}


pub fn donate(sender: &str, text: &str) -> Result<Command, SimpleError> {
    let split = text.split_whitespace().collect::<Vec<&str>>();
    if split.len() < 2 {
        bail!("Expected a at least 2 arguments")
    }
    let amount =  try_with!(split[1].parse::<u64>(), "Could not parse value");
    Ok(Command::Donate { sender: String::from(sender),
                         amount })
}

pub fn party() -> Result<Command, SimpleError> {
    Ok(Command::Party {})
}

pub fn version() -> Result<Command, SimpleError> {
    Ok(Command::Version { })
}

impl CommandReply {

    pub fn text_only(text: &str) -> CommandReply {
        CommandReply {
            text: Some(text.to_string()),
            image: None
        }
    }

    pub fn new(text: &str, image: Vec<u8>) -> CommandReply {
        CommandReply {
            text: Some(text.to_string()),
            image: Some(image)
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.text.is_some() && !self.image.is_some()
    }
}

