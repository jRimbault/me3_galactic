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
    /// specific mission identifier
    mission: Option<String>,
}

fn main() {
    let args = Args::from_args();
    if args.cookie.is_none() {
        eprintln!("The {} cookie is required", galactic::ID_COOKIE);
        std::process::exit(1);
    }
    let cookie = args.cookie.as_deref().unwrap();
    let client = galactic::N7Client::with(&cookie);

    match args.mission.as_deref() {
        Some(system) => launch(&args, &client, &galactic::Mission(system)),
        None => {
            for mission in galactic::MISSIONS.one_hour.iter() {
                launch(&args, &client, mission);
            }
        }
    }
}

fn launch(args: &Args, client: &galactic::N7Client, mission: &galactic::Mission) {
    #[derive(Debug, serde::Deserialize)]
    struct N7Response {
        result: bool,
        ratings: Option<GalaxyStatus>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct GalaxyStatus {
        inner: f64,
        terminus: f64,
        earth: f64,
        outer: f64,
        attican: f64,
    }

    match client.mission(&mission, &args.action) {
        Ok(r) => {
            // this means we've been redirected and the cookie might be expired
            if r.url().as_str() == "http://n7hq.masseffect.com/" {
                eprintln!(
                    "failed {} for {}, cookie might be expired",
                    args.action, mission
                );
            } else if r.status() == 200 {
                eprintln!("{} for {} {}", args.action, mission, r.status());
                eprintln!("{:?}", r.json::<N7Response>().unwrap());
            } else {
                eprintln!("failed {} for {} {}", args.action, mission, r.status());
            }
        }
        Err(error) => eprintln!("failed {} {} {}", args.action, mission, error),
    }
}
