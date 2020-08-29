use galactic::Action;
use structopt::StructOpt;

/// Deploy missions and collect the rewards for galactic readiness in Mass Effect 3.
/// You have to get the value of your identifier cookie on the website, and it expires
/// in a few hours. But running this program once or twice a day should be enough.
#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(subcommand)]
    action: Action,
    /// identifier cookie for n7hq.masseffect.com
    #[structopt(short, long, env = galactic::ID_COOKIE, hide_env_values = true)]
    cookie: Option<String>,
}

fn main() {
    let args = Args::from_args();
    if args.cookie.is_none() {
        eprintln!("The {} cookie is required", galactic::ID_COOKIE);
        std::process::exit(1);
    }
    let cookie = args.cookie.as_deref().unwrap();
    let client = galactic::N7Client::with_cookie(cookie);
    for (result, mission) in galactic::MISSIONS
        .one_hour
        .iter()
        .map(|m| (client.launch_mission((*m, args.action)), m))
    {
        match result {
            Ok(response) => match args.action {
                Action::Deploy => {
                    println!("Deployed fleets to {}", mission);
                }
                Action::Collect => {
                    println!(
                        "Galactic readiness: {} (collected {})",
                        response.readiness().unwrap(),
                        mission
                    );
                }
            },
            Err(error) => println!("{:?}", error),
        }
    }
}
