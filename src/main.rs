use galactic::{Action, Galaxy, N7Client};
use structopt::StructOpt;

/// Deploy missions and collect the rewards for galactic readiness in Mass Effect 3.
/// You have to get the value of your identifier cookie on the website, and it expires
/// in a few hours. But running this program once or twice a day should be enough.
#[derive(Debug, StructOpt)]
struct Args {
    /// identifier cookie for n7hq.masseffect.com
    #[structopt(short, long, env = galactic::ID_COOKIE, hide_env_values = true)]
    cookie: String,
    /// automatic mode, launch this as a background process
    #[structopt(short, long)]
    automatic: bool,
}

#[cfg(debug_assertions)]
const LOG_LEVEL: &str = concat!(env!("CARGO_PKG_NAME"), "=debug");
#[cfg(not(debug_assertions))]
const LOG_LEVEL: &str = "info";

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL)).init();
    let args = Args::from_args();
    let client = N7Client::with_cookie(&args.cookie);
    if args.automatic {
        while let Some(remained) = match_cycle(&client) {
            log::info!("waiting for {}", indicatif::HumanDuration(remained));
            std::thread::sleep(remained);
        }
        std::process::exit(1);
    } else if match_cycle(&client).is_none() {
        std::process::exit(1);
    }
}

fn match_cycle(client: &N7Client) -> Option<std::time::Duration> {
    match cycle(&client) {
        Ok(galaxy) => {
            log::info!("{:#}", galaxy.status);
            galaxy
                .missions
                .iter()
                .max_by_key(|m| m.remained)
                .and_then(|m| m.remained.to_std().ok())
        }
        Err(error) => {
            log::error!("{:#}", error);
            None
        }
    }
}

fn cycle(client: &N7Client) -> anyhow::Result<Galaxy> {
    log::info!("fetching galaxy's status");
    let galaxy = client.status()?;
    log::info!(
        "{} current missions, {} completed",
        galaxy.missions.len(),
        galaxy.missions.iter().filter(|m| m.is_completed).count()
    );
    if galaxy.missions.len() != 0 {
        for mission in galaxy.missions.iter().filter(|m| m.is_completed) {
            log::info!("collecting and deploying mission {}", mission.name);
            mission.collect_and_deploy(&client)?;
        }
    } else {
        for mission in galaxy.raw.one_hour_missions() {
            log::info!("deploying mission {}", mission);
            client.launch_mission((mission.as_ref(), Action::Deploy))?;
        }
    }
    client.status()
}
