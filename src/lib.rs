use structopt::StructOpt;

const BASE_URL: &str = "http://n7hq.masseffect.com/galaxy_at_war/galactic_readiness/";
pub const SYSTEMS: [&str; 5] = ["cyone", "trident", "garvug", "gembat", "pinnacle"];
pub const ID_COOKIE: &str = "ME3N7HQSID";

#[derive(Debug, StructOpt, Clone, Copy)]
pub enum Action {
    /// collect rewards
    Collect,
    /// deploy fleets
    Deploy,
}

impl Action {
    const COLLECT: &'static str = "collectRewards";
    const DEPLOY: &'static str = "deployFleet";
}

pub struct N7Client {
    client: reqwest::blocking::Client,
    action: Action,
}

type ReqResult = reqwest::Result<reqwest::blocking::Response>;

impl N7Client {
    pub fn with(cookie: &str, action: Action) -> Self {
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
            action,
        }
    }

    pub fn mission(&self, system: &str) -> ReqResult {
        action(&self.client, system, &self.action.to_string())
    }
}

fn action(client: &reqwest::blocking::Client, system: &str, action: &str) -> ReqResult {
    client
        .post(BASE_URL)
        .query(&[("ajax_action", action)])
        .form(&[("mission_code", system)])
        .send()
}

impl std::str::FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "collect" | "c" => Ok(Self::Collect),
            "deploy" | "d" => Ok(Self::Collect),
            _ => Err("must be either 'deploy' or 'collect'"),
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
