impl super::PlayerMission {
    pub fn launch(&self, client: &super::N7Client) -> anyhow::Result<()> {
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