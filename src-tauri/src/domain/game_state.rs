pub enum GameState {
    None,
    Login,
}

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait GameStateRepository: shaku::Interface {
    async fn get_game_state(&self) -> GameState;
}
