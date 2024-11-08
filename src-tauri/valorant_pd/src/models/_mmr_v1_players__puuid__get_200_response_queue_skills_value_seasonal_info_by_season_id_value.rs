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
pub struct MmrV1PlayersPuuidGet200ResponseQueueSkillsValueSeasonalInfoBySeasonIdValue {
    /// Season ID
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    #[serde(rename = "NumberOfWins")]
    pub number_of_wins: f64,
    #[serde(rename = "NumberOfWinsWithPlacements")]
    pub number_of_wins_with_placements: f64,
    #[serde(rename = "NumberOfGames")]
    pub number_of_games: f64,
    #[serde(rename = "Rank")]
    pub rank: f64,
    #[serde(rename = "CapstoneWins")]
    pub capstone_wins: f64,
    #[serde(rename = "LeaderboardRank")]
    pub leaderboard_rank: f64,
    #[serde(rename = "CompetitiveTier")]
    pub competitive_tier: f64,
    #[serde(rename = "RankedRating")]
    pub ranked_rating: f64,
    #[serde(rename = "WinsByTier", deserialize_with = "Option::deserialize")]
    pub wins_by_tier: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "GamesNeededForRating")]
    pub games_needed_for_rating: f64,
    #[serde(rename = "TotalWinsNeededForRank")]
    pub total_wins_needed_for_rank: f64,
}

impl MmrV1PlayersPuuidGet200ResponseQueueSkillsValueSeasonalInfoBySeasonIdValue {
    pub fn new(
        season_id: String,
        number_of_wins: f64,
        number_of_wins_with_placements: f64,
        number_of_games: f64,
        rank: f64,
        capstone_wins: f64,
        leaderboard_rank: f64,
        competitive_tier: f64,
        ranked_rating: f64,
        wins_by_tier: Option<std::collections::HashMap<String, f64>>,
        games_needed_for_rating: f64,
        total_wins_needed_for_rank: f64,
    ) -> MmrV1PlayersPuuidGet200ResponseQueueSkillsValueSeasonalInfoBySeasonIdValue {
        MmrV1PlayersPuuidGet200ResponseQueueSkillsValueSeasonalInfoBySeasonIdValue {
            season_id,
            number_of_wins,
            number_of_wins_with_placements,
            number_of_games,
            rank,
            capstone_wins,
            leaderboard_rank,
            competitive_tier,
            ranked_rating,
            wins_by_tier,
            games_needed_for_rating,
            total_wins_needed_for_rank,
        }
    }
}
