use super::GalaxyStatus;
use scraper::{Html, Selector};
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    MissingRef(String),
    PercentError(super::percent::PercentError),
}

pub fn parse_from_html(html: &str) -> Result<GalaxyStatus, ParseError> {
    let document = Html::parse_document(&html);
    let get = |selector: &str| {
        document
            .select(&Selector::parse(&selector).unwrap())
            .next()
            .ok_or_else(|| ParseError::MissingRef(selector.to_owned()))
            .map(|d| d.inner_html())
            .and_then(|p| p.trim_matches('%').parse().map_err(Into::into))
    };
    let status = GalaxyStatus {
        inner: get("#gaw-trating-inner")?,
        terminus: get("#gaw-trating-terminus")?,
        earth: get("#gaw-trating-earth")?,
        outer: get("#gaw-trating-outer")?,
        attican: get("#gaw-trating-attican")?,
    };
    Ok(status)
}

impl From<super::percent::PercentError> for ParseError {
    fn from(e: super::percent::PercentError) -> Self {
        Self::PercentError(e)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PercentError(error) => Some(error),
            _ => None,
        }
    }
}

#[test]
fn test_parsing() {
    let status = parse_from_html(include_str!("../tests/response.html")).unwrap();
    assert_eq!(status.inner.0, 0.9939);
    assert_eq!(status.terminus.0, 0.9939);
    assert_eq!(status.earth.0, 0.9939);
    assert_eq!(status.outer.0, 0.9939);
    assert_eq!(status.attican.0, 0.9939);
}
