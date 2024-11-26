#[macro_use]
extern crate diesel;
extern crate simple_error;
extern crate qrcode_generator;

mod lnbits_client;
mod config;
mod matrix_bot;
mod data_layer;
mod application_service;

use log::LevelFilter;
use crate::config::config::{config_from_cmd, Config};
use crate::data_layer::data_layer::DataLayer;

use crate::lnbits_client::lnbits_client::LNBitsClient;
use crate::matrix_bot::matrix_bot::MatrixBot;

use simple_logger::SimpleLogger;
use std::str::FromStr;
use simple_error::{SimpleError, try_with};

#[tokio::main]
async fn main() -> Result<(), SimpleError>  {

    let config = config_from_cmd();

    SimpleLogger::new().with_utc_timestamps()
                       .with_level(LevelFilter::from_str(config.debug_level.as_str()).unwrap())
                       .init().unwrap();

    log::info!("Starting up.");

    let data_layer =  DataLayer::new(&config);

    let ln_client = LNBitsClient::new(&config);

    let matrix_bot = try_with!(MatrixBot::new( data_layer, ln_client, &config).await,
                                        "Matrix bot could not be constructed");

    matrix_bot.init().await;

    Ok(try_with!(matrix_bot.sync().await,
                           "Syncying failed"))
}
