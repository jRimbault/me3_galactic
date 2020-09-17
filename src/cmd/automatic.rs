use super::N7Cookie;
use crate::client::N7HqClient;
use reqwest::blocking::Client;
use std::path::PathBuf;
use structopt::StructOpt;

/// refresh every missions untils the cookie expires
#[derive(Debug, StructOpt)]
pub struct Automatic {
    /// run in the background automatically
    #[structopt(short, long)]
    daemonize: bool,
    /// for daemon mode, default will not to log
    #[structopt(short, long, parse(from_os_str))]
    log_file: Option<PathBuf>,
}

impl Automatic {
    pub fn run(self, cookie: N7Cookie) {
        if self.daemonize {
            std::process::exit(self.daemonize(cookie));
        }
        let client = Client::with_cookie(&cookie.value);
        while let Some(duration_left) = inner_loop(&client) {
            log::info!(
                "waiting for {}",
                indicatif::FormattedDuration(duration_left)
            );
            std::thread::sleep(duration_left);
        }
        std::process::exit(1);
    }

    fn daemonize(self, cookie: N7Cookie) -> i32 {
        match relaunch_itself(self.log_file, cookie) {
            Ok(_) => 0,
            Err(error) => {
                log::error!("{:#}", error);
                1
            }
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

fn relaunch_itself(log_file: Option<PathBuf>, cookie: N7Cookie) -> anyhow::Result<()> {
    let program = std::env::args_os().next().expect("program name");
    let mut cmd = std::process::Command::new(program);
    cmd.env(crate::ID_COOKIE, &cookie.value).arg("automatic");
    if let Some(log_file) = log_file.as_deref() {
        cmd.stderr(std::fs::File::create(&log_file)?);
    } else {
        cmd.stderr(std::process::Stdio::null());
    }
    // not waiting on it, just dropping it for the OS to maybe pick up later..
    let _ = cmd.spawn()?;
    Ok(())
}
