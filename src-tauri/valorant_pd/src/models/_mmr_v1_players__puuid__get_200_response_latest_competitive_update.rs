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
pub struct MmrV1PlayersPuuidGet200ResponseLatestCompetitiveUpdate {
    /// Match ID
    #[serde(rename = "MatchID")]
    pub match_id: String,
    /// Map ID
    #[serde(rename = "MapID")]
    pub map_id: String,
    /// Season ID
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    #[serde(rename = "MatchStartTime")]
    pub match_start_time: f64,
    #[serde(rename = "TierAfterUpdate")]
    pub tier_after_update: f64,
    #[serde(rename = "TierBeforeUpdate")]
    pub tier_before_update: f64,
    #[serde(rename = "RankedRatingAfterUpdate")]
    pub ranked_rating_after_update: f64,
    #[serde(rename = "RankedRatingBeforeUpdate")]
    pub ranked_rating_before_update: f64,
    #[serde(rename = "RankedRatingEarned")]
    pub ranked_rating_earned: f64,
    #[serde(rename = "RankedRatingPerformanceBonus")]
    pub ranked_rating_performance_bonus: f64,
    #[serde(rename = "CompetitiveMovement")]
    pub competitive_movement: CompetitiveMovement,
    #[serde(rename = "AFKPenalty")]
    pub afk_penalty: f64,
}

impl MmrV1PlayersPuuidGet200ResponseLatestCompetitiveUpdate {
    pub fn new(match_id: String, map_id: String, season_id: String, match_start_time: f64, tier_after_update: f64, tier_before_update: f64, ranked_rating_after_update: f64, ranked_rating_before_update: f64, ranked_rating_earned: f64, ranked_rating_performance_bonus: f64, competitive_movement: CompetitiveMovement, afk_penalty: f64) -> MmrV1PlayersPuuidGet200ResponseLatestCompetitiveUpdate {
        MmrV1PlayersPuuidGet200ResponseLatestCompetitiveUpdate {
            match_id,
            map_id,
            season_id,
            match_start_time,
            tier_after_update,
            tier_before_update,
            ranked_rating_after_update,
            ranked_rating_before_update,
            ranked_rating_earned,
            ranked_rating_performance_bonus,
            competitive_movement,
            afk_penalty,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompetitiveMovement {
    #[serde(rename = "MOVEMENT_UNKNOWN")]
    MovementUnknown,
}

impl Default for CompetitiveMovement {
    fn default() -> CompetitiveMovement {
        Self::MovementUnknown
    }
}

