use crate::domain::player;

#[derive(shaku::Component)]
#[shaku(interface = player::PlayerRepository)]
pub struct PlayerRepositoryImpl;

impl player::PlayerRepository for PlayerRepositoryImpl {
    fn get_player(&self) -> Result<player::Player, String> {
        Ok(player::Player {})
    }
}
