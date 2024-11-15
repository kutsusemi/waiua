shaku::module! {
    pub AppModule {
        components = [crate::infra::repository_impl::player::PlayerRepositoryImpl, crate::infra::repository_impl::game_state::GameStateRepositoryImpl, crate::infra::valorant::ValorantAPIImpl],
        providers = []
    }
}
