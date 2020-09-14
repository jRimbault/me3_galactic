use env_logger::Env;
use structopt::StructOpt;

#[cfg(debug_assertions)]
const LOG_LEVEL: &str = concat!(env!("CARGO_PKG_NAME"), "=debug");
#[cfg(not(debug_assertions))]
const LOG_LEVEL: &str = "info";

fn main() {
    env_logger::from_env(Env::default().default_filter_or(LOG_LEVEL)).init();
    match galactic::Command::from_iter_safe(std::env::args_os()) {
        Ok(cmd) => cmd.run(),
        Err(_) => match galactic::Refresh::from_iter_safe(std::env::args_os()) {
            Ok(cmd) => cmd.run(),
            Err(_) => {
                galactic::Command::clap().print_help().unwrap();
                println!();
            }
        },
    }
}
