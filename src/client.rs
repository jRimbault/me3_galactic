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

    /// consumes both the client and iterator
    pub fn launch_missions<M, I>(
        self,
        missions: I,
    ) -> impl Iterator<Item = anyhow::Result<super::N7Response>>
    where
        M: Into<Mission>,
        I: IntoIterator<Item = M>,
    {
        missions
            .into_iter()
            .map(move |mission| self.launch_mission(mission))
    }

    fn launch_mission<M>(&self, mission: M) -> anyhow::Result<super::N7Response>
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
}

impl<'a> From<(super::Mission<'a>, super::Action)> for Mission {
    fn from((mission, action): (super::Mission<'a>, super::Action)) -> Self {
        Self {
            name: mission.0.to_string(),
            action,
        }
    }
}
