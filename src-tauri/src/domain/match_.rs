use super::error::DomainError;

pub struct Match {
    blue_party_id: i32,
    red_party_id: i32,
}

#[cfg_attr(test, mockall::automock)]
pub trait MatchRepository: shaku::Interface {
    fn get_current_match(&self, match_id: i32) -> Result<Option<Match>, DomainError>;
}
