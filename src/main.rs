use galactic::Action;
use structopt::StructOpt;

/// Deploy missions and collect the rewards for galactic readiness in Mass Effect 3.
/// You have to get the value of your identifier cookie on the website, and it expires
/// in a few hours. But running this program once or twice a day should be enough.
#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(subcommand)]
    action: Option<Action>,
    /// identifier cookie for n7hq.masseffect.com
    #[structopt(short, long, env = galactic::ID_COOKIE, hide_env_values = true)]
    cookie: String,
}

fn main() {
    let args = Args::from_args();
    let client = galactic::N7Client::with_cookie(&args.cookie);
    if let Some(action) = args.action {
        for (result, mission) in galactic::MISSIONS
            .one_hour
            .iter()
            .map(|m| (client.launch_mission((*m, action)), m))
        {
            match result {
                Ok(_) => match action {
                    Action::Deploy => println!("Deployed fleets to {}", mission),
                    Action::Collect => println!("Collected rewards for {}", mission),
                },
                Err(error) => println!("{:?}", error),
            }
        }
    }
    match client.status() {
        Ok(galaxy) => {
            println!("{}", galaxy.status);
            for (name, mission) in galaxy.missions.0 {
                if mission.remained.num_seconds() > 60 {
                    println!(
                        "{} {}m{}s",
                        name,
                        mission.remained.num_minutes(),
                        mission.remained.num_seconds() % 60
                    );
                } else {
                    println!("{} {}s", name, mission.remained.num_seconds());
                }
            }
        }
        Err(error) => println!("{:?}", error),
    }
}
