mod client;
mod cmd;
mod data;
mod html;
mod percent;

pub use cmd::{Automatic, Command, N7Cookie, Refresh};
use percent::Percentage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BASE_URL: &str = "http://n7hq.masseffect.com/galaxy_at_war/galactic_readiness/";
const ID_COOKIE: &str = "ME3N7HQSID";

#[derive(Debug, Clone, Copy)]
enum Action {
    /// collect rewards
    Collect,
    /// deploy fleets
    Deploy,
}

#[derive(Debug, Clone)]
struct Mission {
    name: String,
    action: Action,
}

#[derive(Debug, Default)]
struct CurrentMissions(Vec<PlayerMission>);

#[derive(Default, Debug)]
struct Galaxy {
    status: GalaxyStatus,
    missions: CurrentMissions,
    raw: Data,
}

#[derive(Debug)]
struct PlayerMission {
    name: String,
    start: chrono::DateTime<chrono::Local>,
    duration: chrono::Duration,
    is_completed: bool,
    remained: chrono::Duration,
}

#[derive(Debug, serde::Deserialize)]
struct N7Response {
    result: bool,
    ratings: Option<GalaxyStatus>,
}

#[derive(Debug, serde::Deserialize, Default)]
struct GalaxyStatus {
    inner: Percentage,
    terminus: Percentage,
    earth: Percentage,
    outer: Percentage,
    attican: Percentage,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Data {
    theaters: Theaters,
    missions: HashMap<String, MissionDef>,
    player_missions: EventualMissions,
    lang: HashMap<String, String>,
}

/// The json is either a empty list `[]` or an object `{}` if not empty.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum EventualMissions {
    NoMission([i32; 0]),
    Missions(HashMap<String, RawPlayerMission>),
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct MissionDef {
    theater: String,
    planet: String,
    name: String,
    description: String,
    complete_msg: String,
    duration: i64,
    rating: String,
}

impl Default for EventualMissions {
    fn default() -> Self {
        Self::Missions(Default::default())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RawPlayerMission {
    start: i64,
    duration: i64,
    is_completed: bool,
    remained: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Theaters {
    inner: Theater,
    terminus: Theater,
    earth: Theater,
    outer: Theater,
    attican: Theater,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Theater {
    name: String,
}
