use std::str::FromStr;
use lnurl::lightning_address::LightningAddress;
use lnurl::lnurl::LnUrl;

pub fn parse_lnurl(input: &str) -> Option<LnUrl> {
    match LnUrl::from_str(input) {
        Ok(lnurl) => Some(lnurl),
        Err(_) => match LightningAddress::from_str(input) {
            Ok(lightning_address) => Some(lightning_address.lnurl()),
            Err(_) => None
        },
    }
}
