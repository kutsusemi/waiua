use shaku::HasComponent;
use tauri::State;

use crate::{domain::player::PlayerRepository, module::AppModule};

use super::state::{MyState, MyStateImpl};

pub fn world_exec<'r>(state: impl MyState<'r, AppModule>) -> Result<super::dto::Player, String> {
    let player_repository: &dyn PlayerRepository = state.state().resolve_ref();
    Ok(player_repository.get_player()?.into())
}
#[tauri::command]
pub fn world(state: State<'_, AppModule>) -> Result<(), String> {
    let s = MyStateImpl::new(state);
    world_exec(s);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::commands::dto;
    use crate::commands::state::tests::MyStateMock;
    #[test]
    fn should_return_0_with_no_loaded_models() {
        let container = AppModule::builder().build();
        assert_eq!(
            world_exec(MyStateMock::new(&container)).unwrap(),
            dto::Player {}
        );
    }
}
