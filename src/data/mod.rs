mod fmt;

use std::collections::HashMap;

impl crate::Action {
    pub(crate) const fn value(&self) -> &'static str {
        match *self {
            Self::Collect => "collectRewards",
            Self::Deploy => "deployFleet",
        }
    }

    pub(crate) const fn description(&self) -> &'static str {
        match *self {
            Self::Collect => "collect rewards",
            Self::Deploy => "deploy fleet",
        }
    }
}

impl crate::Data {
    pub(crate) fn one_hour_missions(&self) -> impl Iterator<Item = &String> {
        self.missions
            .iter()
            .filter_map(|(n, m)| if m.duration == 3600 { Some(n) } else { None })
    }
}

impl From<(&str, super::Action)> for crate::Mission {
    fn from((name, action): (&str, super::Action)) -> Self {
        Self {
            name: name.to_string(),
            action,
        }
    }
}

impl From<&HashMap<String, crate::RawPlayerMission>> for crate::CurrentMissions {
    fn from(missions: &HashMap<String, crate::RawPlayerMission>) -> Self {
        Self(missions.iter().map(Into::into).collect())
    }
}

impl From<(&String, &crate::RawPlayerMission)> for crate::PlayerMission {
    fn from((name, mission): (&String, &crate::RawPlayerMission)) -> Self {
        fn i64_to_datetime(timestamp: i64) -> chrono::DateTime<chrono::Local> {
            chrono::DateTime::<chrono::Utc>::from_utc(
                chrono::NaiveDateTime::from_timestamp(timestamp, 0),
                chrono::Utc,
            )
            .into()
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

impl std::ops::Deref for crate::CurrentMissions {
    type Target = Vec<crate::PlayerMission>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
