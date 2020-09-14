const REDIRECTED_URL_BOUND: usize = 27;

pub(crate) trait N7HqClient {
    fn with_cookie(cookie: &str) -> Self;
    fn status(&self) -> anyhow::Result<crate::Galaxy>;
    fn launch_mission<M>(&self, mission: M) -> anyhow::Result<super::N7Response>
    where
        M: Into<crate::Mission>;

    fn refresh_missions(&self) -> anyhow::Result<crate::Galaxy> {
        log::info!("fetching galaxy's status");
        let galaxy = self.status()?;
        log::info!(
            "{} current missions, {} completed",
            galaxy.missions.len(),
            galaxy.missions.iter().filter(|m| m.is_completed).count()
        );
        if galaxy.missions.len() != 0 {
            for mission in galaxy.missions.iter().filter(|m| m.is_completed) {
                log::info!("collecting and deploying mission {}", mission.name);
                self.collect_and_deploy(&mission)?;
            }
        } else {
            for mission in galaxy.raw.one_hour_missions() {
                log::info!("deploying mission {}", mission);
                self.launch_mission((mission.as_ref(), super::Action::Deploy))?;
            }
        }
        self.status()
    }

    fn collect_and_deploy(&self, mission: &crate::PlayerMission) -> anyhow::Result<()> {
        if !mission.is_completed {
            return Ok(());
        }
        self.launch_mission(crate::Mission {
            name: mission.name.clone(),
            action: crate::Action::Collect,
        })?;
        if mission.duration.num_hours() > 1 {
            log::warn!(
                "the {} mission is longer than one hour, this isn't optimal, you should manually launch a one hour mission in this sector",
                mission.name
            );
            return Ok(());
        }
        self.launch_mission(crate::Mission {
            name: mission.name.clone(),
            action: crate::Action::Deploy,
        })?;
        Ok(())
    }
}

impl N7HqClient for reqwest::blocking::Client {
    fn with_cookie(cookie: &str) -> Self {
        use super::ID_COOKIE;
        use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
        log::debug!("building agent with {}={:?}", ID_COOKIE, cookie);
        Self::builder()
            .cookie_store(true)
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert(
                    COOKIE,
                    HeaderValue::from_str(&format!("{}={}", ID_COOKIE, cookie)).unwrap(),
                );
                headers
            })
            .build()
            .unwrap()
    }

    fn launch_mission<M>(&self, mission: M) -> anyhow::Result<super::N7Response>
    where
        M: Into<crate::Mission>,
    {
        let mission = mission.into();
        log::debug!("{} {}", mission.action, mission.name);
        let response = self
            .post(super::BASE_URL)
            .query(&[("ajax_action", mission.action.value())])
            .form(&[("mission_code", &mission.name)])
            .send()?;
        if response.status() != 200 {
            Err(anyhow::anyhow!("unknown, {}", response.status())
                .context(format!("failed {} for {}", mission.action, mission.name)))
        } else if is_redirected(response.url()) {
            Err(anyhow::anyhow!("cookie is expired or invalid")
                .context(format!("failed {} for {}", mission.action, mission.name)))
        } else {
            Ok(response.json()?)
        }
    }

    fn status(&self) -> anyhow::Result<crate::Galaxy> {
        log::debug!("fetch galaxy's global status");
        let response = self.get(super::BASE_URL).send()?;
        let html = if is_redirected(response.url()) {
            return Err(
                anyhow::anyhow!("cookie is expired or invalid").context("failed getting data")
            );
        } else {
            response.text()?
        };
        let doc = super::html::Document(scraper::Html::parse_document(&html));
        let data = doc.infos()?;
        Ok(crate::Galaxy {
            status: doc.galaxy_status()?,
            missions: match &data.player_missions {
                crate::EventualMissions::NoMission(_) => Default::default(),
                crate::EventualMissions::Missions(m) => m.into(),
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
