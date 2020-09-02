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
        let response = self
            .client
            .post(super::BASE_URL)
            .query(&[("ajax_action", mission.action.value())])
            .form(&[("mission_code", &mission.name)])
            .send()?;
        if response.status() != 200 {
            Err(anyhow::anyhow!("unknown, {}", response.status())
                .context(format!("failed {} for {}", mission.action, mission.name)))
        } else if is_redirected(response.url()) {
            Err(anyhow::anyhow!("cookie is expired")
                .context(format!("failed {} for {}", mission.action, mission.name)))
        } else {
            Ok(response.json()?)
        }
    }

    pub fn status(&self) -> anyhow::Result<Galaxy> {
        let response = self.client.get(super::BASE_URL).send()?;
        let html = if is_redirected(response.url()) {
            return Err(anyhow::anyhow!("cookie is expired").context("failed getting misc data"));
        } else {
            response.text()?
        };
        let doc = super::html::Document(scraper::Html::parse_document(&html));
        Ok(Galaxy {
            status: doc.galaxy_status()?,
            missions: doc.infos()?.player_missions.get().into(),
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
                .map(|(name, mission)| (name, mission.into()))
                .collect(),
        )
    }
}

impl From<crate::html::script::PlayerMission> for PlayerMission {
    fn from(mission: crate::html::script::PlayerMission) -> Self {
        fn i64_to_datetime(timestamp: i64) -> chrono::DateTime<chrono::Utc> {
            chrono::DateTime::from_utc(
                chrono::NaiveDateTime::from_timestamp(timestamp, 0),
                chrono::Utc,
            )
        }

        fn u64_to_duration(duration: u64) -> chrono::Duration {
            chrono::Duration::from_std(std::time::Duration::from_secs(duration)).unwrap()
        }
        Self {
            start: i64_to_datetime(mission.start),
            duration: u64_to_duration(mission.duration as _),
            remained: u64_to_duration(if mission.remained < 0 {
                0
            } else {
                mission.remained as _
            }),
            is_completed: mission.is_completed,
        }
    }
}

impl AsRef<HashMap<String, PlayerMission>> for CurrentMissions {
    fn as_ref(&self) -> &HashMap<String, PlayerMission> {
        &self.0
    }
}
