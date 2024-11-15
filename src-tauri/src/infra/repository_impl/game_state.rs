use std::sync::Arc;

use crate::domain::game_state;

#[derive(shaku::Component)]
#[shaku(interface = game_state::GameStateRepository)]
pub struct GameStateRepositoryImpl {
    #[shaku(inject)]
    valorant_api: Arc<dyn crate::infra::valorant::ValorantAPI>,
}

#[async_trait::async_trait]
impl game_state::GameStateRepository for GameStateRepositoryImpl {
    async fn get_game_state(&self) -> game_state::GameState {
        match self.valorant_api.get_game_state().await {
            Ok(_) => game_state::GameState::Login,
            Err(_) => game_state::GameState::None,
        }
    }
}
