use structopt::StructOpt;

/// Deploy missions and collect the rewards for galactic readiness in Mass Effect 3.
/// You have to get the value of your identifier cookie on the website, and it expires
/// in a few hours. But running this program once or twice a day should be enough.
#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(flatten)]
    action: galactic::Action,
    /// identifier cookie for n7hq.masseffect.com
    #[structopt(short, long, env = galactic::ID_COOKIE)]
    cookie: Option<String>,
    /// specific mission identifier
    mission: Option<String>,
}

fn main() {
    let args = Args::from_args();
    if args.cookie.is_none() {
        eprintln!("The {} cookie is required", galactic::ID_COOKIE);
        std::process::exit(1);
    }
    let cookie = args.cookie.clone().unwrap();
    let client = galactic::N7Client::with(&cookie, args.action);

    match args.mission.clone() {
        Some(system) => mission(&args, &client, &system),
        None => {
            for system in galactic::SYSTEMS.iter() {
                mission(&args, &client, system);
            }
        }
    }
}

fn mission(args: &Args, client: &galactic::N7Client, system: &str) {
    match client.mission(&system) {
        Ok(r) => {
            // this means we've been redirected and the cookie might be expired
            if r.url().as_str() == "http://n7hq.masseffect.com/" {
                eprintln!("failed {} {}, cookie might be expired", args.action, system);
            } else {
                eprintln!("{} {} {}", args.action, system, r.status())
            }
        }
        Err(error) => eprintln!("failed {} {} {}", args.action, system, error),
    }
}
