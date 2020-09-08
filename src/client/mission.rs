impl super::PlayerMission {
    pub fn collect_and_deploy(&self, client: &super::N7Client) -> anyhow::Result<()> {
        if !self.is_completed {
            return Ok(());
        }
        client.launch_mission(super::Mission {
            name: self.name.clone(),
            action: crate::Action::Collect,
        })?;
        if self.duration.num_hours() > 1 {
            log::warn!(
                "the {} mission is longer than one hour, this isn't optimal, you should manually launch a one hour mission in this sector",
                self.name
            );
            return Ok(());
        }
        client.launch_mission(super::Mission {
            name: self.name.clone(),
            action: crate::Action::Deploy,
        })?;
        Ok(())
    }
}
