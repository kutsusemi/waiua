use super::error::DomainError;

pub enum GameState {
    None,
    Login,
}

#[async_trait::async_trait]
pub trait GameStateRepository: shaku::Interface {
    async fn get_game_state(&self) -> GameState;
}
