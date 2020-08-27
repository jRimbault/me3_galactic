use structopt::StructOpt;

#[derive(Clone, Debug)]
pub struct Mission<'a>(pub &'a str);
#[derive(Clone, Debug)]
pub struct Missions<'a> {
    pub one_hour: [Mission<'a>; 5],
}

const BASE_URL: &str = "http://n7hq.masseffect.com/galaxy_at_war/galactic_readiness/";
pub const ID_COOKIE: &str = "ME3N7HQSID";
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

impl Action {
    const COLLECT: &'static str = "collectRewards";
    const DEPLOY: &'static str = "deployFleet";

    pub fn description(&self) -> &'static str {
        match *self {
            Self::Collect => "collect rewards",
            Self::Deploy => "deploy fleets",
        }
    }
}

pub struct N7Client {
    client: reqwest::blocking::Client,
}

type ReqResult = reqwest::Result<reqwest::blocking::Response>;

impl N7Client {
    pub fn with(cookie: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::COOKIE,
            reqwest::header::HeaderValue::from_str(&format!("{}={}", ID_COOKIE, cookie)).unwrap(),
        );
        Self {
            client: reqwest::blocking::Client::builder()
                .cookie_store(true)
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub fn mission(&self, system: &Mission, action: &Action) -> ReqResult {
        self.client
            .post(BASE_URL)
            .query(&[("ajax_action", action.to_string())])
            .form(&[("mission_code", system.0)])
            .send()
    }
}

impl std::str::FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "collect" | "c" => Ok(Self::Collect),
            "deploy" | "d" => Ok(Self::Collect),
            _ => Err("must be either 'collect' or 'deploy' ('c'/'d')"),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Collect => f.write_str(Self::COLLECT),
            Self::Deploy => f.write_str(Self::DEPLOY),
        }
    }
}

impl std::fmt::Display for Mission<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
