mod client;
mod html;
mod percent;

pub use client::{Galaxy, Mission as N7Mission, N7Client, PlayerMission};
pub use percent::Percentage;
use std::fmt;

const BASE_URL: &str = "http://n7hq.masseffect.com/galaxy_at_war/galactic_readiness/";
pub const ID_COOKIE: &str = "ME3N7HQSID";

#[derive(Copy, Clone, Debug)]
pub struct Mission<'a>(pub &'a str);

#[derive(Debug, Clone, Copy)]
pub enum Action {
    /// collect rewards
    Collect,
    /// deploy fleets
    Deploy,
}

#[derive(Debug, serde::Deserialize)]
pub struct N7Response {
    pub result: bool,
    pub ratings: Option<GalaxyStatus>,
}

#[derive(Debug, serde::Deserialize, Default)]
pub struct GalaxyStatus {
    pub inner: Percentage,
    pub terminus: Percentage,
    pub earth: Percentage,
    pub outer: Percentage,
    pub attican: Percentage,
}

impl Action {
    const fn value(&self) -> &'static str {
        match *self {
            Self::Collect => "collectRewards",
            Self::Deploy => "deployFleet",
        }
    }

    const fn description(&self) -> &'static str {
        match *self {
            Self::Collect => "collect rewards",
            Self::Deploy => "deploy fleet",
        }
    }
}

impl N7Response {
    pub fn readiness(&self) -> Option<Percentage> {
        self.ratings.as_ref().map(|r| r.readiness())
    }
}

impl GalaxyStatus {
    pub fn readiness(&self) -> Percentage {
        let total = self.inner.0 + self.terminus.0 + self.earth.0 + self.outer.0 + self.attican.0;
        Percentage(total / 5.)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl fmt::Display for Mission<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for GalaxyStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "Inner: {:.2}, ", self.inner)?;
            write!(f, "Terminus: {:.2}, ", self.terminus)?;
            write!(f, "Earth: {:.2}, ", self.earth)?;
            write!(f, "Outer: {:.2}, ", self.outer)?;
            write!(f, "Attican: {:.2}", self.terminus)
        } else {
            writeln!(f, "Sector statuses:")?;
            writeln!(f, "  Inner: {:.2}", self.inner)?;
            writeln!(f, "  Terminus: {:.2}", self.terminus)?;
            writeln!(f, "  Earth: {:.2}", self.earth)?;
            writeln!(f, "  Outer: {:.2}", self.outer)?;
            write!(f, "  Attican: {:.2}", self.terminus)
        }
    }
}
