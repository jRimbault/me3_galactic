impl super::PlayerMission {
    pub fn collect_and_deploy(&self, client: &super::N7Client) -> anyhow::Result<()> {
        if self.duration.num_hours() > 1 {
            log::warn!(
                "the {} mission is longer than one hour, this isn't optimal",
                self.name
            );
        }
        if self.is_completed {
            client.launch_mission(super::Mission {
                name: self.name.clone(),
                action: crate::Action::Collect,
            })?;
            client.launch_mission(super::Mission {
                name: self.name.clone(),
                action: crate::Action::Deploy,
            })?;
        }
        Ok(())
    }
}
