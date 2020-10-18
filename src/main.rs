use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(subcommand)]
    command: Option<galactic::Command>,
    #[structopt(flatten)]
    cookie: galactic::N7Cookie,
    #[structopt(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> anyhow::Result<()> {
    let args = get_args();
    env_logger::Builder::new()
        .filter_level(args.verbosity.log_level().unwrap().to_level_filter())
        .try_init()?;
    match args.command {
        Some(cmd) => cmd.run(args.cookie),
        None => galactic::Refresh::from_args().run(args.cookie),
    };
    Ok(())
}

fn get_args() -> Args {
    let mut args = Args::from_args();
    args.verbosity.set_default(Some(log::Level::Info));
    args
}
