pub mod data;

use super::GalaxyStatus;
use scraper::{Html, Selector};

pub struct Document(pub(crate) Html);

impl Document {
    pub fn galaxy_status(&self) -> anyhow::Result<GalaxyStatus> {
        let get = |selector: &str| {
            self.0
                .select(&Selector::parse(&selector).unwrap())
                .next()
                .ok_or_else(|| anyhow::anyhow!(format!("missing ref {}", selector)))
                .map(|d| d.inner_html())
                .and_then(|p| Ok(p.trim_matches('%').parse()?))
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

    pub fn infos(&self) -> anyhow::Result<data::Data> {
        let selector = Selector::parse("script").unwrap();
        let javascript = self
            .0
            .select(&selector)
            .find_map(|n| {
                let text = n.inner_html();
                if text.contains("$gawdata") {
                    Some(text)
                } else {
                    None
                }
            })
            .unwrap();
        let start = javascript
            .find('{')
            .ok_or_else(|| anyhow::anyhow!("missing start of json data"))?;
        let end = javascript
            .rfind('}')
            .ok_or_else(|| anyhow::anyhow!("missing end of json data"))?;
        Ok(serde_json::from_str(&javascript[start..end + 1])?)
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
                "../../tests/script.html"
            )));
            let script = doc.infos().unwrap();
            println!("{:#?}", script);
        }

        #[test]
        fn format() {
            let s: data::Data =
                serde_json::from_str(include_str!("../../tests/script.json")).unwrap();
            println!("{:#?}", s);
        }
    }
}
