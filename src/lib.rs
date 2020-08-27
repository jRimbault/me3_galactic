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

pub struct N7Client(reqwest::blocking::Client);

type ReqResult = reqwest::Result<reqwest::blocking::Response>;

impl N7Client {
    pub fn with(cookie: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::COOKIE,
            reqwest::header::HeaderValue::from_str(&format!("{}={}", ID_COOKIE, cookie)).unwrap(),
        );
        Self(
            reqwest::blocking::Client::builder()
                .cookie_store(true)
                .default_headers(headers)
                .build()
                .unwrap(),
        )
    }

    pub fn mission(&self, system: &Mission, action: &Action) -> ReqResult {
        self.0
            .post(BASE_URL)
            .query(&[("ajax_action", action.value())])
            .form(&[("mission_code", system.0)])
            .send()
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
