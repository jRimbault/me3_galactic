use galactic::client::{make_client, N7Client};
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

    let executor = N7Client::builder().client(make_client(cookie)).build();

    let missions = galactic::MISSIONS
        .one_hour
        .iter()
        .map(|m| (m.clone(), args.action.clone()));

    for result in executor.run_missions(missions) {
        match result {
            Ok(response) => println!("{:?}", response),
            Err(error) => println!("{:?}", error),
        }
    }
}
