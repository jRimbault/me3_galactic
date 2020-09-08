pub mod mission;

use crate::html::data;
use std::collections::HashMap;

const REDIRECTED_URL_BOUND: usize = 27;

#[derive(Debug)]
pub struct N7Client {
    agent: ureq::Agent,
}

#[derive(Debug, Clone)]
pub struct Mission {
    pub name: String,
    pub action: super::Action,
}

#[derive(Debug, Default)]
pub struct CurrentMissions(pub(crate) Vec<PlayerMission>);

#[derive(Default, Debug)]
pub struct Galaxy {
    pub status: super::GalaxyStatus,
    pub missions: CurrentMissions,
    pub raw: data::Data,
}

#[derive(Debug)]
pub struct PlayerMission {
    pub name: String,
    start: chrono::DateTime<chrono::Utc>,
    duration: chrono::Duration,
    pub is_completed: bool,
    pub remained: chrono::Duration,
}

impl N7Client {
    pub fn with_cookie(cookie: &str) -> Self {
        use super::ID_COOKIE;
        log::debug!("building agent with {}={:?}", ID_COOKIE, cookie);
        Self {
            agent: {
                let agent = ureq::Agent::new();
                let cookie = ureq::Cookie::build(ID_COOKIE.to_owned(), cookie.to_owned())
                    .domain("n7hq.masseffect.com")
                    .path("/")
                    .secure(false)
                    .http_only(true)
                    .finish();
                agent.set_cookie(cookie);
                agent
            },
        }
    }

    pub fn launch_mission<M>(&self, mission: M) -> anyhow::Result<super::N7Response>
    where
        M: Into<Mission>,
    {
        let mission = mission.into();
        log::debug!("{} {}", mission.action, mission.name);
        let response = self
            .agent
            .post(super::BASE_URL)
            .query("ajax_action", mission.action.value())
            .send_form(&[("mission_code", &mission.name)]);
        if response.status() != 200 {
            Err(anyhow::anyhow!("unknown, {}", response.status())
                .context(format!("failed {} for {}", mission.action, mission.name)))
        } else if is_redirected(&response.get_url()) {
            Err(anyhow::anyhow!("cookie is expired or invalid")
                .context(format!("failed {} for {}", mission.action, mission.name)))
        } else {
            Ok(serde_json::from_reader(response.into_reader())?)
        }
    }

    pub fn status(&self) -> anyhow::Result<Galaxy> {
        log::debug!("fetch galaxy's global status");
        let response = self.agent.get(super::BASE_URL).call();
        let html = if is_redirected(&response.get_url()) {
            return Err(
                anyhow::anyhow!("cookie is expired or invalid").context("failed getting data")
            );
        } else {
            response.into_string()?
        };
        let doc = super::html::Document(scraper::Html::parse_document(&html));
        let data = doc.infos()?;
        Ok(Galaxy {
            status: doc.galaxy_status()?,
            missions: match &data.player_missions {
                data::EventualMissions::NoMission(_) => Default::default(),
                data::EventualMissions::Missions(m) => m.into(),
            },
            raw: data,
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

impl<'a> From<(&'a str, super::Action)> for Mission {
    fn from((mission, action): (&'a str, super::Action)) -> Self {
        Self {
            name: mission.to_string(),
            action,
        }
    }
}

impl From<&HashMap<String, data::PlayerMission>> for CurrentMissions {
    fn from(missions: &HashMap<String, data::PlayerMission>) -> Self {
        CurrentMissions(missions.iter().map(Into::into).collect())
    }
}

impl From<(&String, &data::PlayerMission)> for PlayerMission {
    fn from((name, mission): (&String, &data::PlayerMission)) -> Self {
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
            name: name.to_owned(),
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

impl std::ops::Deref for CurrentMissions {
    type Target = Vec<PlayerMission>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
