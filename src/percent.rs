use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Percentage(f64);

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
        let f = value.trim_matches('"').parse::<f64>()?;
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

use std::fmt;
impl fmt::Display for PercentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
