use serde::Deserialize;
use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Percentage(pub(crate) f64);

#[derive(Debug)]
pub enum PercentError {
    Value(Ordering),
    Parse(std::num::ParseFloatError),
}

impl TryFrom<f64> for Percentage {
    type Error = Ordering;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0f64 {
            Err(Ordering::Less)
        } else if value > 1f64 {
            if value > 100f64 {
                Err(Ordering::Greater)
            } else {
                Ok(Self(value / 100f64))
            }
        } else {
            Ok(Self(value))
        }
    }
}

impl FromStr for Percentage {
    type Err = PercentError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let f = value.parse::<f64>()?;
        f.try_into().map_err(Into::into)
    }
}

impl From<std::num::ParseFloatError> for PercentError {
    fn from(pfe: std::num::ParseFloatError) -> Self {
        Self::Parse(pfe)
    }
}

impl From<Ordering> for PercentError {
    fn from(pfe: Ordering) -> Self {
        Self::Value(pfe)
    }
}

impl fmt::Display for PercentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.)
    }
}

impl<'de> Deserialize<'de> for Percentage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value
            .trim_matches('"')
            .parse::<Percentage>()
            .map_err(serde::de::Error::custom)
    }
}

impl std::error::Error for PercentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            _ => None,
        }
    }
}
