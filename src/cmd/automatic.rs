use crate::client::N7HqClient;
use reqwest::blocking::Client;
use structopt::StructOpt;
/// refresh every missions untils the cookie expires
#[derive(Debug, StructOpt)]
pub struct Automatic {
    #[structopt(flatten)]
    args: super::Args,
}

impl Automatic {
    pub fn run(self) {
        let client = Client::with_cookie(&self.args.cookie);
        while let Some(duration_left) = inner_loop(&client) {
            log::info!(
                "waiting for {}",
                indicatif::FormattedDuration(duration_left)
            );
            std::thread::sleep(duration_left);
        }
        std::process::exit(1);
    }
}

fn inner_loop(client: &Client) -> Option<std::time::Duration> {
    match client.refresh_missions() {
        Ok(galaxy) => {
            log::info!("{:#}", galaxy.status);
            let missions_duration = galaxy
                .missions
                .iter()
                .max_by_key(|m| m.remained)
                .and_then(|m| m.remained.to_std().ok())
                .unwrap();
            Some(missions_duration)
        }
        Err(error) => {
            log::error!("{:#}", error);
            None
        }
    }
}
