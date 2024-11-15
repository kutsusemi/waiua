shaku::module! {
    pub AppModule {
        components = [crate::infra::repository_impl::player::PlayerRepositoryImpl, crate::infra::repository_impl::game_state::GameStateRepositoryImpl, crate::infra::repository_impl::match_::MatchRepositoryImpl, crate::infra::valorant::ValorantAPIImpl],
        providers = []
    }
}

pub mod test {
    use shaku::ModuleBuilder;

    use super::*;
    pub fn create_mock_module_builder() -> ModuleBuilder<AppModule> {
        AppModule::builder()
            .with_component_override::<dyn crate::domain::player::PlayerRepository>(Box::new(
                crate::domain::player::MockPlayerRepository::new(),
            ))
            .with_component_override::<dyn crate::domain::game_state::GameStateRepository>(
                Box::new(crate::domain::game_state::MockGameStateRepository::new()),
            )
            .with_component_override::<dyn crate::domain::match_::MatchRepository>(Box::new(
                crate::domain::match_::MockMatchRepository::new(),
            ))
            .with_component_override::<dyn crate::infra::valorant::ValorantAPI>(Box::new(
                crate::infra::valorant::MockValorantAPIImpl::new(),
            ))
    }
}
