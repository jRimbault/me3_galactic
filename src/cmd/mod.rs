mod automatic;
mod refresh;

pub use automatic::Automatic;
pub use refresh::Refresh;
use structopt::StructOpt;

/// Deploy missions and collect the rewards for galactic readiness in Mass Effect 3.
/// You have to get the value of your identifier cookie on the website, and it expires
/// in a few hours. But running this program once or twice a day should be enough.
#[derive(Debug, StructOpt)]
pub enum Command {
    Refresh(Refresh),
    Automatic(Automatic),
}

#[derive(Debug, StructOpt)]
pub struct N7Cookie {
    /// identifier cookie for n7hq.masseffect.com
    #[structopt(short = "c", long = "cookie", env = crate::ID_COOKIE, hide_env_values = true)]
    value: String,
}

impl Command {
    pub fn run(self, cookie: N7Cookie) {
        match self {
            Self::Refresh(a) => a.run(cookie),
            Self::Automatic(a) => a.run(cookie),
        }
    }
}
