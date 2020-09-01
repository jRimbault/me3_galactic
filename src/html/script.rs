use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Infos {
    theaters: Theaters,
    missions: HashMap<String, Mission>,
    pub player_missions: EventualMissions,
    lang: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventualMissions {
    NoMission(Vec<PlayerMission>),
    Missions(HashMap<String, PlayerMission>),
}

impl EventualMissions {
    pub fn get(self) -> HashMap<String, PlayerMission> {
        match self {
            Self::NoMission(_) => Default::default(),
            Self::Missions(m) => m,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mission {
    theater: String,
    planet: String,
    name: String,
    description: String,
    complete_msg: String,
    duration: i64,
    rating: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerMission {
    pub start: i64,
    pub duration: i64,
    pub is_completed: bool,
    pub remained: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Theaters {
    inner: Theater,
    terminus: Theater,
    earth: Theater,
    outer: Theater,
    attican: Theater,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Theater {
    name: String,
}
