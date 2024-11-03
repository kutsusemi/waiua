
pub struct Player {

}

pub trait PlayerRepository: shaku::Interface{
    fn get_player(&self) -> Result<Player,String>;
}