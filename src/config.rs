pub mod config {

    use clap::{Arg, App};

    #[derive(Clone, Debug)]
    pub struct Config {
        pub matrix_server: String,
        pub matrix_password: String,
        pub lnbits_url: String,
        pub lnbits_x_api_key: String,
        pub database_url: String,
        pub debug_level: String,
        pub donate_user: String,
        pub btc_donation_address: String
    }

    impl Config {
        pub fn new(matrix_server: &str,
               matrix_password: &str,
               lnbits_url: &str,
               lnbits_x_api_key: &str,
               database_url: &str,
               debug_level: &str,
               donate_user: &str,
               btc_donation_address: &str) -> Config {
            Config {
                matrix_server: matrix_server.to_string(),
                matrix_password: matrix_password.to_string(),
                lnbits_url: lnbits_url.to_string(),
                lnbits_x_api_key: lnbits_x_api_key.to_string(),
                database_url: database_url.to_string(),
                debug_level: debug_level.to_string(),
                donate_user: donate_user.to_string(),
                btc_donation_address: btc_donation_address.to_string()
            }
        }
    }

    pub fn config_from_cmd() -> Config {
        let args = wild::args_os();
        let args = argfile::expand_args_from(
            args,
            argfile::parse_fromfile,
            argfile::PREFIX,
        ).unwrap();

        let matches = App::new("LN-Matrix-Bot")
            .version("0.1.0")
            .author("AE")
            .about("LN-Matrix-Bot")
            .arg(Arg::with_name("matrix-server")
                .long("matrix-server")
                .takes_value(true)
                .required(true)
                .help("Server"))
            .arg(Arg::with_name("matrix-password")
                .long("matrix-password")
                .takes_value(true)
                .required(true)
                .help("Bot password"))
            .arg(Arg::with_name("lnbits-url")
                .long("lnbits-url")
                .takes_value(true)
                .required(true)
                .help("lnbits url"))
            .arg(Arg::with_name("lnbits-x-api-key")
                .long("lnbits-x-api-key")
                .takes_value(true)
                .required(true)
                .help("lnbits x api key"))
            .arg(Arg::with_name("database-url")
                .long("database-url")
                .takes_value(true)
                .required(true)
                .help("database url"))
            .arg(Arg::with_name("debug-level")
                .long("debug-level")
                .takes_value(true)
                .default_value("Info")
                .required(false)
                .help("debugging level"))
            .arg(Arg::with_name("donate-user")
                .long("donate-user")
                .takes_value(true)
                .required(true)
                .help("The user receiving any donations"))
            .arg(Arg::with_name("btc-donation-address")
                .long("btc-donation-address")
                .takes_value(true)
                .required(true)
                .help("The BTC address to display for donations"))

            .get_matches_from(args);

        let matrix_server = matches.value_of("matrix-server").unwrap();

        let matrix_password = matches.value_of("matrix-password").unwrap();

        let lnbits_url = matches.value_of("lnbits-url").unwrap();

        let lnbits_x_api_key = matches.value_of("lnbits-x-api-key").unwrap();

        let database_url = matches.value_of("database-url").unwrap();

        let debug_level = matches.value_of("debug-level").unwrap();

        let donate_user = matches.value_of("donate-user").unwrap();

        let btc_donation_address = matches.value_of("btc-donation-address").unwrap();

        Config::new(matrix_server,
                    matrix_password,
                    lnbits_url,
                    lnbits_x_api_key,
                    database_url,
                    debug_level,
                    donate_user,
                    btc_donation_address)
    }
}
