use galactic::Action;
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

fn main() -> anyhow::Result<()> {
    let args = Args::from_args();
    let client = galactic::N7Client::with_cookie(&args.cookie);
    if args.automatic {
        loop {
            cycle(&client)?;
            std::thread::sleep(std::time::Duration::from_secs(3600));
        }
    } else {
        cycle(&client)?;
    }
    Ok(())
}

fn cycle(client: &galactic::N7Client) -> anyhow::Result<()> {
    let galaxy = client.status()?;
    if galaxy.missions.len() != 0 {
        for mission in galaxy.missions.iter() {
            mission.launch(&client)?;
        }
    } else {
        for mission in galaxy.raw.one_hour_missions() {
            client.launch_mission((mission.as_ref(), Action::Deploy))?;
        }
    }
    let galaxy = client.status()?;
    println!("{}", galaxy.status);
    for mission in galaxy.missions.iter() {
        println!("{}", mission);
    }
    Ok(())
}
