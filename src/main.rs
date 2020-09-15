use env_logger::Env;
use structopt::StructOpt;

#[cfg(debug_assertions)]
const LOG_LEVEL: &str = concat!(env!("CARGO_PKG_NAME"), "=debug");
#[cfg(not(debug_assertions))]
const LOG_LEVEL: &str = "info";

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(subcommand)]
    command: Option<galactic::Command>,
    #[structopt(flatten)]
    cookie: galactic::N7Cookie,
}

fn main() {
    env_logger::from_env(Env::default().default_filter_or(LOG_LEVEL)).init();
    let args = Args::from_args();
    match args.command {
        Some(cmd) => cmd.run(),
        None => galactic::Refresh::from_args().run(),
    }
}
