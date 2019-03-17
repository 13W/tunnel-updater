#[derive(Clone)]
pub struct Config {
    pub port: u32,
    pub host: String,
    pub secret: String,
}

use clap::*;

pub fn get() -> Config {
    let matches = clap::App::new("IP tunnel updater")
        .version(crate_version!())
        .about("Tools for updating the remote tunnel address")

        .arg(clap::Arg::with_name("address")
            .help("host address")
            .short("a")
            .env("ADDRESS")
            .default_value("0.0.0.0"))

        .arg(clap::Arg::with_name("port")
            .help("host port")
            .short("p")
            .env("PORT")
            .default_value("38123"))

        .arg(clap::Arg::with_name("secret")
            .help("update key")
            .short("s")
            .env("SECRET")
            .default_value("SeCrEtKeY"))
        .get_matches();

    let host = matches.value_of("address").unwrap().to_string();
    let port = matches.value_of("port").unwrap()
        .parse::<u32>().expect("invalid port number");
    let secret = matches.value_of("secret").unwrap().to_string();

    Config {
        host,
        port,
        secret,
    }
}
