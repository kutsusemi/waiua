use crate::domain::player;

#[derive(shaku::Component)]
#[shaku(interface = player::PlayerRepository)]
pub struct PlayerRepositoryImpl;

impl player::PlayerRepository for PlayerRepositoryImpl {
    fn get_party_of_main_player(
        &self,
    ) -> Result<Vec<player::Player>, crate::domain::error::DomainError> {
        todo!()
    }

    fn get_party(
        &self,
        party_id: i32,
    ) -> Result<Vec<player::Player>, crate::domain::error::DomainError> {
        todo!()
    }
}
