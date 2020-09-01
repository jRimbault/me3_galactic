pub mod script;

use super::GalaxyStatus;
use scraper::{Html, Selector};
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    MissingRef(String),
    PercentError(super::percent::PercentError),
}

pub struct Document(pub(crate) Html);

impl Document {
    pub fn galaxy_status(&self) -> Result<GalaxyStatus, ParseError> {
        let get = |selector: &str| {
            self.0
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

    pub fn infos(&self) -> serde_json::Result<script::Infos> {
        let selector = Selector::parse("script").unwrap();
        let javascript = self
            .0
            .select(&selector)
            .find_map(|n| {
                let text = n.inner_html();
                if text.contains("var $gawdata") {
                    Some(text)
                } else {
                    None
                }
            })
            .unwrap();
        let s = javascript.trim_end_matches("; $gawdata.is_mobile = false;");
        let s = s.trim_start_matches("var $gawdata = ");
        serde_json::from_str(s)
    }
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

#[cfg(test)]
mod tests {
    mod parsing {
        use super::super::*;

        #[test]
        fn galaxy_status() {
            let doc = Document(Html::parse_document(include_str!(
                "../../tests/response.html"
            )));
            let status = doc.galaxy_status().unwrap();
            assert_eq!(status.inner.0, 0.9939);
            assert_eq!(status.terminus.0, 0.9939);
            assert_eq!(status.earth.0, 0.9939);
            assert_eq!(status.outer.0, 0.9939);
            assert_eq!(status.attican.0, 0.9939);
        }

        #[test]
        fn mission_status() {
            let doc = Document(Html::parse_document(include_str!(
                "../../tests/full_response.html"
            )));
            let script = doc.infos().unwrap();
            println!("{:#?}", script);
        }

        #[test]
        fn format() {
            let s: script::Infos =
                serde_json::from_str(include_str!("../../tests/script.json")).unwrap();
            println!("{:#?}", s);
        }
    }
}
