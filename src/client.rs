const REDIRECTED_URL_BOUND: usize = 27;

#[derive(Debug)]
pub struct N7Client {
    client: reqwest::blocking::Client,
}

#[derive(Debug, Clone)]
pub struct Mission {
    pub name: String,
    pub action: super::Action,
}

impl N7Client {
    pub fn with_cookie(cookie: &str) -> Self {
        use super::ID_COOKIE;
        use reqwest::{
            blocking::Client,
            header::{HeaderMap, HeaderValue, COOKIE},
        };
        Self {
            client: Client::builder()
                .cookie_store(true)
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        COOKIE,
                        HeaderValue::from_str(&format!("{}={}", ID_COOKIE, cookie)).unwrap(),
                    );
                    headers
                })
                .build()
                .unwrap(),
        }
    }

    pub fn launch_mission<M>(&self, mission: M) -> anyhow::Result<super::N7Response>
    where
        M: Into<Mission>,
    {
        let redirected_url = &super::BASE_URL[..REDIRECTED_URL_BOUND];
        let mission = mission.into();
        self.client
            .post(super::BASE_URL)
            .query(&[("ajax_action", mission.action.value())])
            .form(&[("mission_code", &mission.name)])
            .send()
            .map_err(Into::into)
            .and_then(|r| {
                if r.status() != 200 {
                    Err(anyhow::anyhow!("unknown, {}", r.status())
                        .context(format!("failed {} for {}", mission.action, mission.name)))
                } else if r.url().as_str() == redirected_url {
                    Err(anyhow::anyhow!("cookie might be expired")
                        .context(format!("failed {} for {}", mission.action, mission.name)))
                } else {
                    r.json().map_err(Into::into)
                }
            })
    }

    pub fn status(&self) -> anyhow::Result<super::GalaxyStatus> {
        self.client
            .get(super::BASE_URL)
            .send()
            .map_err(Into::into)
            .and_then(|r| r.text().map_err(Into::into))
            .map(|html| {
                let document = scraper::Html::parse_document(&html);
                let selector = scraper::Selector::parse("div.gaw-trating").unwrap();
                let mut iter = document.select(&selector);
                super::GalaxyStatus {
                    inner: iter
                        .next()
                        .map(|d| d.inner_html())
                        .and_then(|p| p.trim_matches('%').parse().ok())
                        .unwrap(),
                    terminus: iter
                        .next()
                        .map(|d| d.inner_html())
                        .and_then(|p| p.trim_matches('%').parse().ok())
                        .unwrap(),
                    earth: iter
                        .next()
                        .map(|d| d.inner_html())
                        .and_then(|p| p.trim_matches('%').parse().ok())
                        .unwrap(),
                    outer: iter
                        .next()
                        .map(|d| d.inner_html())
                        .and_then(|p| p.trim_matches('%').parse().ok())
                        .unwrap(),
                    attican: iter
                        .next()
                        .map(|d| d.inner_html())
                        .and_then(|p| p.trim_matches('%').parse().ok())
                        .unwrap(),
                }
            })
    }
}

impl<'a> From<(super::Mission<'a>, super::Action)> for Mission {
    fn from((mission, action): (super::Mission<'a>, super::Action)) -> Self {
        Self {
            name: mission.0.to_string(),
            action,
        }
    }
}
