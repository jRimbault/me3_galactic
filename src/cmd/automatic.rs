use crate::client::N7HqClient;
use reqwest::blocking::Client;
use structopt::StructOpt;
/// refresh every missions untils the cookie expires
#[derive(Debug, StructOpt)]
pub struct Automatic {
    #[structopt(flatten)]
    cookie: super::N7Cookie,
    /// run in the background automatically
    #[structopt(short, long)]
    daemonize: bool,
    /// for daemon mode, default will not to log
    #[structopt(short, long, parse(from_os_str))]
    log_file: Option<std::path::PathBuf>,
}

impl Automatic {
    pub fn run(self) {
        if self.daemonize {
            self.daemonize();
        }
        let client = Client::with_cookie(&self.cookie.value);
        while let Some(duration_left) = inner_loop(&client) {
            log::info!(
                "waiting for {}",
                indicatif::FormattedDuration(duration_left)
            );
            std::thread::sleep(duration_left);
        }
        std::process::exit(1);
    }

    fn daemonize(self) -> ! {
        let program: std::path::PathBuf = std::env::args_os().next().unwrap().into();
        let mut cmd = std::process::Command::new(program);
        cmd.env(crate::ID_COOKIE, &self.cookie.value)
            .arg("automatic");
        if let Some(log_file) = self.log_file.as_deref() {
            cmd.stderr(std::fs::File::create(&log_file).unwrap());
        }
        // not waiting on it, just dropping it for the OS to maybe pick up later..
        match cmd.spawn() {
            Err(error) => {
                log::error!("couldn't launch program, {:#}", anyhow::Error::from(error));
                std::process::exit(1);
            }
            Ok(_) => std::process::exit(0),
        }
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
