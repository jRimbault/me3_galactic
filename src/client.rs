use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct N7Client {
    client: ClientWithCookie,
}

#[derive(Debug)]
pub struct ClientWithCookie(reqwest::blocking::Client);

#[derive(Debug, Clone)]
pub struct Mission {
    name: String,
    action: super::Action,
}

impl N7Client {
    /// consumes both the client and iterator
    pub fn run_missions<M, I>(
        self,
        missions: I,
    ) -> impl Iterator<Item = anyhow::Result<super::N7Response>>
    where
        M: Into<Mission>,
        I: IntoIterator<Item = M>,
    {
        missions.into_iter().map(Into::into).map(move |mission| {
            self.client
                .0
                .post(super::BASE_URL)
                .query(&[("ajax_action", mission.action.value())])
                .form(&[("mission_code", &mission.name)])
                .send()
                .map_err(|e| e.into())
                .and_then(|r| {
                    if r.status() != 200 {
                        Err(anyhow::anyhow!("unknown, {}", r.status())
                            .context(format!("failed {} for {}", mission.action, mission.name)))
                    } else if r.url().as_str() == "http://n7hq.masseffect.com/" {
                        Err(anyhow::anyhow!("cookie might be expired")
                            .context(format!("failed {} for {}", mission.action, mission.name)))
                    } else {
                        r.json().map_err(Into::into)
                    }
                })
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

pub fn make_client(cookie: &str) -> ClientWithCookie {
    use super::ID_COOKIE;
    use reqwest::{
        blocking::Client,
        header::{HeaderMap, HeaderValue, COOKIE},
    };

    ClientWithCookie(
        Client::builder()
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
    )
}
