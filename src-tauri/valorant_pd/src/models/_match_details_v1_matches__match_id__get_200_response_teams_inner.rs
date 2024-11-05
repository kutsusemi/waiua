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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponseTeamsInner {
    #[serde(rename = "teamId")]
    pub team_id: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerTeamId>,
    #[serde(rename = "won")]
    pub won: bool,
    #[serde(rename = "roundsPlayed")]
    pub rounds_played: f64,
    #[serde(rename = "roundsWon")]
    pub rounds_won: f64,
    #[serde(rename = "numPoints")]
    pub num_points: f64,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponseTeamsInner {
    pub fn new(
        team_id: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerTeamId,
        won: bool,
        rounds_played: f64,
        rounds_won: f64,
        num_points: f64,
    ) -> MatchDetailsV1MatchesMatchIdGet200ResponseTeamsInner {
        MatchDetailsV1MatchesMatchIdGet200ResponseTeamsInner {
            team_id: Box::new(team_id),
            won,
            rounds_played,
            rounds_won,
            num_points,
        }
    }
}
