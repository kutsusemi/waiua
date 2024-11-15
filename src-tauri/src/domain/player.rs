use super::error::DomainError;

pub struct Player {
    pub is_main_player: bool,
}

pub trait PlayerRepository: shaku::Interface {
    fn get_party_of_main_player(&self) -> Result<Vec<Player>, DomainError>;
    fn get_party(&self, party_id: i32) -> Result<Vec<Player>, DomainError>;
}
