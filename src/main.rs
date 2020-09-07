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

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::from_args();
    let client = N7Client::with_cookie(&args.cookie);
    if args.automatic {
        loop {
            if match_cycle(&client).is_err() {
                break;
            }
            countdown(std::time::Duration::from_secs(3600));
        }
        std::process::exit(1);
    } else if match_cycle(&client).is_err() {
        std::process::exit(1);
    }
}

fn match_cycle(client: &N7Client) -> Result<(), ()> {
    match cycle(&client) {
        Ok(galaxy) => {
            log::info!("{:#}", galaxy.status);
            Ok(())
        }
        Err(error) => {
            log::error!("{:#}", error);
            Err(())
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
            client.launch_mission((mission.as_ref(), Action::Deploy))?;
        }
    }
    client.status()
}

fn countdown(wait_for: std::time::Duration) {
    use std::thread;
    use std::time::{Duration, Instant};
    let spinner = indicatif::ProgressBar::new_spinner();
    let start_time = Instant::now();
    let one_second = Duration::from_secs(1);
    let sleep = Duration::from_millis(100);
    for _ in 0.. {
        let elapsed = start_time.elapsed();
        if elapsed > wait_for {
            break;
        }
        let remaining = wait_for - elapsed;
        if remaining < one_second {
            break;
        }
        spinner.tick();
        spinner.set_message(&format!(
            "waiting for {}",
            indicatif::HumanDuration(remaining)
        ));
        thread::sleep(sleep);
    }
}
