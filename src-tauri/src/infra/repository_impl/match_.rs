use crate::domain::{
    error::DomainError,
    match_::{self, Match},
};

#[derive(shaku::Component)]
#[shaku(interface = match_::MatchRepository)]
pub struct MatchRepositoryImpl;

impl match_::MatchRepository for MatchRepositoryImpl {
    fn get_current_match(&self, match_id: i32) -> Result<Option<Match>, DomainError> {
        todo!()
    }
}
