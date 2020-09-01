use std::collections::HashMap;

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

#[derive(Debug)]
pub struct CurrentMissions(pub HashMap<String, PlayerMission>);

#[derive(Debug)]
pub struct Galaxy {
    pub status: super::GalaxyStatus,
    pub missions: CurrentMissions,
}

#[derive(Debug)]
pub struct PlayerMission {
    start: chrono::DateTime<chrono::Utc>,
    duration: chrono::Duration,
    is_completed: bool,
    pub remained: chrono::Duration,
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
                } else if is_redirected(r.url()) {
                    Err(anyhow::anyhow!("cookie is expired")
                        .context(format!("failed {} for {}", mission.action, mission.name)))
                } else {
                    r.json().map_err(Into::into)
                }
            })
    }

    pub fn status(&self) -> anyhow::Result<Galaxy> {
        self.client
            .get(super::BASE_URL)
            .send()
            .map_err(Into::into)
            .and_then(|r| {
                if is_redirected(r.url()) {
                    Err(anyhow::anyhow!("cookie is expired").context("failed getting misc data"))
                } else {
                    r.text().map_err(Into::into)
                }
            })
            .and_then(|html| {
                let doc = super::html::Document(scraper::Html::parse_document(&html));
                Ok(Galaxy {
                    status: doc.galaxy_status()?,
                    missions: doc.infos()?.player_missions.get().into(),
                })
            })
    }
}

/// instead of doing a 40X when the auth has expired, they redirect to the basename url
fn is_redirected<S: AsRef<str>>(url: &S) -> bool {
    let redirected_url = &super::BASE_URL[..REDIRECTED_URL_BOUND];
    url.as_ref() == redirected_url
}

impl<'a> From<(super::Mission<'a>, super::Action)> for Mission {
    fn from((mission, action): (super::Mission<'a>, super::Action)) -> Self {
        Self {
            name: mission.0.to_string(),
            action,
        }
    }
}

impl From<HashMap<String, crate::html::script::PlayerMission>> for CurrentMissions {
    fn from(missions: HashMap<String, crate::html::script::PlayerMission>) -> Self {
        CurrentMissions(
            missions
                .into_iter()
                .map(|(name, mission)| {
                    (
                        name,
                        PlayerMission {
                            start: chrono::DateTime::<chrono::Utc>::from_utc(
                                chrono::NaiveDateTime::from_timestamp(mission.start, 0),
                                chrono::Utc,
                            ),
                            duration: chrono::Duration::from_std(std::time::Duration::from_secs(
                                mission.duration as _,
                            ))
                            .unwrap(),
                            remained: chrono::Duration::from_std(std::time::Duration::from_secs(
                                if mission.remained < 0 {
                                    0
                                } else {
                                    mission.remained as _
                                },
                            ))
                            .unwrap(),
                            is_completed: mission.is_completed,
                        },
                    )
                })
                .collect(),
        )
    }
}

impl AsRef<HashMap<String, PlayerMission>> for CurrentMissions {
    fn as_ref(&self) -> &HashMap<String, PlayerMission> {
        &self.0
    }
}
