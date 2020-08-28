use galactic::client::N7Client;
use structopt::StructOpt;

/// Deploy missions and collect the rewards for galactic readiness in Mass Effect 3.
/// You have to get the value of your identifier cookie on the website, and it expires
/// in a few hours. But running this program once or twice a day should be enough.
#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(subcommand)]
    action: galactic::Action,
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
    let missions = galactic::MISSIONS
        .one_hour
        .iter()
        .map(|m| (m.clone(), args.action.clone()));

    let client = N7Client::with_cookie(cookie);
    for result in client.launch_missions(missions) {
        match result {
            Ok(response) => println!("{:?}", response),
            Err(error) => println!("{:?}", error),
        }
    }
}
