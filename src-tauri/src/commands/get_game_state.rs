use std::future::IntoFuture;

use shaku::HasComponent;

use crate::{domain::game_state::GameStateRepository, module::AppModule};

use super::{
    dto::GameState,
    state::{CommandState, CommandStateImpl},
};

async fn get_game_state_exec<'r>(
    state: impl CommandState<'r, AppModule>,
) -> Result<super::dto::GameState, String> {
    let game_state_repository: &dyn GameStateRepository = state.state().resolve_ref();
    Ok(game_state_repository
        .get_game_state()
        .into_future()
        .await
        .into())
}

#[tauri::command]
pub async fn get_game_state(state: tauri::State<'_, AppModule>) -> Result<GameState, String> {
    let s = CommandStateImpl::new(state);

    get_game_state_exec(s).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{dto::GameState, state::tests};
    #[tokio::test]
    async fn should_return_0_with_no_loaded_models() {
        let container = AppModule::builder().build();
        let app_handle = tests::CommandStateMock::new(&container);
        assert_eq!(get_game_state_exec(app_handle).await, Ok(GameState::Login));
    }
}