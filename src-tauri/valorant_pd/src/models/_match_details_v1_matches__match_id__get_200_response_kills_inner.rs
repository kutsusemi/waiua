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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponseKillsInner {
    /// Time in milliseconds since the start of the game
    #[serde(rename = "gameTime")]
    pub game_time: f64,
    /// Time in milliseconds since the start of the round
    #[serde(rename = "roundTime")]
    pub round_time: f64,
    /// Player UUID
    #[serde(rename = "killer")]
    pub killer: String,
    /// Player UUID
    #[serde(rename = "victim")]
    pub victim: String,
    #[serde(rename = "victimLocation")]
    pub victim_location: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInnerLocation>,
    #[serde(rename = "assistants")]
    pub assistants: Vec<String>,
    #[serde(rename = "playerLocations")]
    pub player_locations: Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInner>,
    #[serde(rename = "finishingDamage")]
    pub finishing_damage: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlayerStatsInnerKillsInnerFinishingDamage>,
    #[serde(rename = "round")]
    pub round: f64,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponseKillsInner {
    pub fn new(game_time: f64, round_time: f64, killer: String, victim: String, victim_location: models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInnerLocation, assistants: Vec<String>, player_locations: Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInner>, finishing_damage: models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlayerStatsInnerKillsInnerFinishingDamage, round: f64) -> MatchDetailsV1MatchesMatchIdGet200ResponseKillsInner {
        MatchDetailsV1MatchesMatchIdGet200ResponseKillsInner {
            game_time,
            round_time,
            killer,
            victim,
            victim_location: Box::new(victim_location),
            assistants,
            player_locations,
            finishing_damage: Box::new(finishing_damage),
            round,
        }
    }
}
