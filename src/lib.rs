mod client;
mod percent;

pub use client::{Mission as N7Mission, N7Client};
pub use percent::Percentage;
use structopt::StructOpt;

const BASE_URL: &str = "http://n7hq.masseffect.com/galaxy_at_war/galactic_readiness/";
pub const ID_COOKIE: &str = "ME3N7HQSID";

#[derive(Copy, Clone, Debug)]
pub struct Mission<'a>(pub &'a str);
#[derive(Copy, Clone, Debug)]
pub struct Missions<'a> {
    /// longer missions don't make econmic sense, but maybe I'll add them someday
    pub one_hour: [Mission<'a>; 5],
}

pub const MISSIONS: Missions = Missions {
    one_hour: [
        Mission("cyone"),
        Mission("trident"),
        Mission("garvug"),
        Mission("gembat"),
        Mission("pinnacle"),
    ],
};

#[derive(Debug, StructOpt, Clone, Copy)]
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

#[derive(Debug, serde::Deserialize)]
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
    fn readiness(&self) -> Percentage {
        let total = self.inner.0 + self.terminus.0 + self.earth.0 + self.outer.0 + self.attican.0;
        Percentage(total / 5.)
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.description())
    }
}

impl std::fmt::Display for Mission<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
