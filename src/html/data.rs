use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    theaters: Theaters,
    pub missions: HashMap<String, Mission>,
    pub player_missions: EventualMissions,
    lang: HashMap<String, String>,
}

impl Data {
    pub fn one_hour_missions(&self) -> impl Iterator<Item = &String> {
        self.missions
            .iter()
            .filter_map(|(n, m)| if m.duration == 3600 { Some(n) } else { None })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventualMissions {
    NoMission(Vec<PlayerMission>),
    Missions(HashMap<String, PlayerMission>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mission {
    theater: String,
    planet: String,
    pub name: String,
    description: String,
    complete_msg: String,
    pub duration: i64,
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
