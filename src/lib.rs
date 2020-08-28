pub mod client;

use structopt::StructOpt;

const BASE_URL: &str = "http://n7hq.masseffect.com/galaxy_at_war/galactic_readiness/";
pub const ID_COOKIE: &str = "ME3N7HQSID";

#[derive(Clone, Debug)]
pub struct Mission<'a>(pub &'a str);
#[derive(Clone, Debug)]
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

#[derive(Debug, StructOpt, Clone)]
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
    #[serde(deserialize_with = "string_as_f64")]
    pub inner: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub terminus: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub earth: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub outer: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub attican: f64,
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

fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = <String as serde::Deserialize>::deserialize(deserializer)?;
    value
        .trim_matches('"')
        .parse::<f64>()
        .map_err(serde::de::Error::custom)
}
