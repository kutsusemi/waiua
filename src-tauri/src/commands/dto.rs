use crate::domain;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct Player {}

impl From<domain::player::Player> for Player {
    fn from(_: domain::player::Player) -> Self {
        Player {}
    }
}

impl Into<domain::player::Player> for Player {
    fn into(self) -> domain::player::Player {
        domain::player::Player {
            is_main_player: false,
        }
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub enum GameState {
    None,
    Login,
}

impl From<domain::game_state::GameState> for GameState {
    fn from(m: domain::game_state::GameState) -> Self {
        match m {
            domain::game_state::GameState::None => GameState::None,
            domain::game_state::GameState::Login => GameState::Login,
        }
    }
}

impl Into<domain::game_state::GameState> for GameState {
    fn into(self) -> domain::game_state::GameState {
        match self {
            GameState::None => domain::game_state::GameState::None,
            GameState::Login => domain::game_state::GameState::Login,
        }
    }
}
