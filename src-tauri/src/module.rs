shaku::module! {
    pub AppModule {
        components = [crate::infra::repository_impl::player::PlayerRepositoryImpl, crate::infra::valorant::ValorantAPIImpl],
        providers = []
    }
}
