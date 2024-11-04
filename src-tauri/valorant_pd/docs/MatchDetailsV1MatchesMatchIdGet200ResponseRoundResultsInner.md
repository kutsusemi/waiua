# MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**round_num** | **f64** |  | 
**round_result** | **String** |  | 
**round_ceremony** | **String** |  | 
**winning_team** | [**models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerTeamId**](_match_details_v1_matches__matchID__get_200_response_players_inner_teamId.md) |  | 
**bomb_planter** | Option<**String**> | Player UUID | [optional]
**bomb_defuser** | Option<[**models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerTeamId**](_match_details_v1_matches__matchID__get_200_response_players_inner_teamId.md)> |  | [optional]
**plant_round_time** | Option<**f64**> | Time in milliseconds since the start of the round when the bomb was planted. 0 if not planted | [optional]
**plant_player_locations** | Option<[**Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInner>**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_plantPlayerLocations_inner.md)> |  | 
**plant_location** | [**models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInnerLocation**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_plantPlayerLocations_inner_location.md) |  | 
**plant_site** | **String** |  | 
**defuse_round_time** | Option<**f64**> | Time in milliseconds since the start of the round when the bomb was defused. 0 if not defused | [optional]
**defuse_player_locations** | Option<[**Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInner>**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_plantPlayerLocations_inner.md)> |  | 
**defuse_location** | [**models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInnerLocation**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_plantPlayerLocations_inner_location.md) |  | 
**player_stats** | [**Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlayerStatsInner>**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_playerStats_inner.md) |  | 
**round_result_code** | **String** | Empty string if the timer expired | 
**player_economies** | Option<[**Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlayerEconomiesInner>**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_playerEconomies_inner.md)> |  | 
**player_scores** | Option<[**Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlayerScoresInner>**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_playerScores_inner.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


