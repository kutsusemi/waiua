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
pub struct PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInner {
    #[serde(rename = "TeamID")]
    pub team_id: Box<models::PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerTeamId>,
    #[serde(rename = "Players")]
    pub players: Vec<models::PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInner>,
}

impl PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInner {
    pub fn new(
        team_id: models::PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerTeamId,
        players: Vec<models::PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInner>,
    ) -> PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInner {
        PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInner {
            team_id: Box::new(team_id),
            players,
        }
    }
}
