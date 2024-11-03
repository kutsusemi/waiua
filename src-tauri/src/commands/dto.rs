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
        domain::player::Player {}
    }
}
