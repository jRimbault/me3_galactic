use crate::client::N7HqClient;
use reqwest::blocking::Client;
use structopt::StructOpt;

/// refresh every missions and quits, this is the default
#[derive(Debug, StructOpt)]
pub struct Refresh {
    // placeholder to allow arg parsing when falling back to refresh
    #[structopt(short = "c", long = "cookie")]
    _value: Option<String>,
}

impl Refresh {
    pub fn run(self, cookie: super::N7Cookie) {
        let client = Client::with_cookie(&cookie.value);
        match client.refresh_missions() {
            Ok(galaxy) => log::info!("{:#}", galaxy.status),
            Err(error) => {
                log::error!("{:#}", error);
                std::process::exit(1);
            }
        }
    }
}
