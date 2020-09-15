use crate::client::N7HqClient;
use reqwest::blocking::Client;
use structopt::StructOpt;

/// refresh every missions and quits, this is the default
#[derive(Debug, StructOpt)]
pub struct Refresh {
    #[structopt(flatten)]
    cookie: super::N7Cookie,
}

impl Refresh {
    pub fn run(self) {
        let client = Client::with_cookie(&self.cookie.value);
        match client.refresh_missions() {
            Ok(galaxy) => log::info!("{:#}", galaxy.status),
            Err(error) => {
                log::error!("{:#}", error);
                std::process::exit(1);
            }
        }
    }
}
