pub mod config {

    use clap::{Arg, Command};

    #[derive(Clone, Debug)]
    pub struct Config {
        pub matrix_server: String,
        pub matrix_username: String,
        pub matrix_password: String,
        pub lnbits_url: String,
        pub lnbits_x_api_key: String,
        pub database_url: String,
        pub debug_level: String,
        pub donate_user: Option<String>,
        pub btc_donation_address: String
    }

    impl Config {
        pub fn new(matrix_server: &str,
                   matrix_username: &str,
               matrix_password: &str,
               lnbits_url: &str,
               lnbits_x_api_key: &str,
               database_url: &str,
               debug_level: &str,
               donate_user: Option<&String>,
               btc_donation_address: &str) -> Config {
            Config {
                matrix_server: matrix_server.to_string(),
                matrix_username: matrix_username.to_string(),
                matrix_password: matrix_password.to_string(),
                lnbits_url: lnbits_url.to_string(),
                lnbits_x_api_key: lnbits_x_api_key.to_string(),
                database_url: database_url.to_string(),
                debug_level: debug_level.to_string(),
                donate_user: donate_user.map(|s| s.to_string()),
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

        let matches = Command::new("LN-Matrix-Bot")
            .version("0.3.0")
            .author("AE")
            .about("LN-Matrix-Bot")
            .arg(Arg::new("matrix-server")
                .long("matrix-server")
                .required(true)
                .help("Server"))
            .arg(Arg::new("matrix-username")
                .long("matrix-username")
                .required(true)
                .help("Bot username"))
            .arg(Arg::new("matrix-password")
                .long("matrix-password")
                .required(true)
                .help("Bot password"))
            .arg(Arg::new("lnbits-url")
                .long("lnbits-url")
                .required(true)
                .help("lnbits url"))
            .arg(Arg::new("lnbits-x-api-key")
                .long("lnbits-x-api-key")
                .required(true)
                .help("lnbits x api key"))
            .arg(Arg::new("database-url")
                .long("database-url")
                .required(true)
                .help("database url"))
            .arg(Arg::new("debug-level")
                .long("debug-level")
                .default_value("Info")
                .required(false)
                .help("debugging level"))
            .arg(Arg::new("donate-user")
                .long("donate-user")
                .required(false)
                .help("The user receiving any donations"))
            .arg(Arg::new("btc-donation-address")
                .long("btc-donation-address")
                .default_value("bc1q72dzh04fwxx780w05twtmn5fxzegpawdn5zg3g")
                .required(false)
                .help("The BTC address to display for donations"))

            .get_matches_from(args);

        let matrix_server = matches.get_one::<String>("matrix-server").unwrap();

        let matrix_username = matches.get_one::<String>("matrix-username").unwrap();

        let matrix_password = matches.get_one::<String>("matrix-password").unwrap();

        let lnbits_url = matches.get_one::<String>("lnbits-url").unwrap();

        let lnbits_x_api_key = matches.get_one::<String>("lnbits-x-api-key").unwrap();

        let database_url = matches.get_one::<String>("database-url").unwrap();

        let debug_level = matches.get_one::<String>("debug-level").unwrap();

        let donate_user = matches.get_one::<String>("donate-user");

        let btc_donation_address = matches.get_one::<String>("btc-donation-address").unwrap();

        Config::new(matrix_server,
                    matrix_username,
                    matrix_password,
                    lnbits_url,
                    lnbits_x_api_key,
                    database_url,
                    debug_level,
                    donate_user,
                    btc_donation_address)
    }
}
