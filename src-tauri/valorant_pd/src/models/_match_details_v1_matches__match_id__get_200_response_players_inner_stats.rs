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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStats {
    #[serde(rename = "score")]
    pub score: f64,
    #[serde(rename = "roundsPlayed")]
    pub rounds_played: f64,
    #[serde(rename = "kills")]
    pub kills: f64,
    #[serde(rename = "deaths")]
    pub deaths: f64,
    #[serde(rename = "assists")]
    pub assists: f64,
    #[serde(rename = "playtimeMillis")]
    pub playtime_millis: f64,
    #[serde(
        rename = "abilityCasts",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ability_casts: Option<
        Option<
            Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStatsAbilityCasts>,
        >,
    >,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStats {
    pub fn new(
        score: f64,
        rounds_played: f64,
        kills: f64,
        deaths: f64,
        assists: f64,
        playtime_millis: f64,
    ) -> MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStats {
        MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStats {
            score,
            rounds_played,
            kills,
            deaths,
            assists,
            playtime_millis,
            ability_casts: None,
        }
    }
}
