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
pub struct ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrants {
    #[serde(rename = "GamePlayed")]
    pub game_played: f64,
    #[serde(rename = "GameWon")]
    pub game_won: f64,
    #[serde(rename = "RoundPlayed")]
    pub round_played: f64,
    #[serde(rename = "RoundWon")]
    pub round_won: f64,
    #[serde(rename = "Missions")]
    pub missions: serde_json::Value,
    #[serde(rename = "Modifier")]
    pub modifier: Box<models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrantsModifier>,
    #[serde(rename = "NumAFKRounds")]
    pub num_afk_rounds: f64,
}

impl ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrants {
    pub fn new(game_played: f64, game_won: f64, round_played: f64, round_won: f64, missions: serde_json::Value, modifier: models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrantsModifier, num_afk_rounds: f64) -> ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrants {
        ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrants {
            game_played,
            game_won,
            round_played,
            round_won,
            missions,
            modifier: Box::new(modifier),
            num_afk_rounds,
        }
    }
}

