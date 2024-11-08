/*
 * Valorant API
 *
 * Valorant API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInner {
    /// Player UUID
    #[serde(rename = "Subject")]
    pub subject: String,
    /// Character ID
    #[serde(rename = "CharacterID")]
    pub character_id: String,
    #[serde(rename = "CharacterSelectionState")]
    pub character_selection_state: CharacterSelectionState,
    #[serde(rename = "PregamePlayerState")]
    pub pregame_player_state: PregamePlayerState,
    #[serde(rename = "CompetitiveTier")]
    pub competitive_tier: f64,
    #[serde(rename = "PlayerIdentity")]
    pub player_identity:
        Box<models::PartiesV1PartiesPartyIdGet200ResponseMembersInnerPlayerIdentity>,
    #[serde(rename = "SeasonalBadgeInfo")]
    pub seasonal_badge_info: Box<
        models::PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInnerSeasonalBadgeInfo,
    >,
    #[serde(rename = "IsCaptain")]
    pub is_captain: bool,
}

impl PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInner {
    pub fn new(
        subject: String,
        character_id: String,
        character_selection_state: CharacterSelectionState,
        pregame_player_state: PregamePlayerState,
        competitive_tier: f64,
        player_identity: models::PartiesV1PartiesPartyIdGet200ResponseMembersInnerPlayerIdentity,
        seasonal_badge_info: models::PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInnerSeasonalBadgeInfo,
        is_captain: bool,
    ) -> PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInner {
        PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInner {
            subject,
            character_id,
            character_selection_state,
            pregame_player_state,
            competitive_tier,
            player_identity: Box::new(player_identity),
            seasonal_badge_info: Box::new(seasonal_badge_info),
            is_captain,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CharacterSelectionState {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "selected")]
    Selected,
    #[serde(rename = "locked")]
    Locked,
}

impl Default for CharacterSelectionState {
    fn default() -> CharacterSelectionState {
        Self::Empty
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PregamePlayerState {
    #[serde(rename = "joined")]
    Joined,
}

impl Default for PregamePlayerState {
    fn default() -> PregamePlayerState {
        Self::Joined
    }
}
