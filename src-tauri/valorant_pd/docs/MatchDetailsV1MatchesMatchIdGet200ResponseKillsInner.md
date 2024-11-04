# MatchDetailsV1MatchesMatchIdGet200ResponseKillsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**game_time** | **f64** | Time in milliseconds since the start of the game | 
**round_time** | **f64** | Time in milliseconds since the start of the round | 
**killer** | **String** | Player UUID | 
**victim** | **String** | Player UUID | 
**victim_location** | [**models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInnerLocation**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_plantPlayerLocations_inner_location.md) |  | 
**assistants** | **Vec<String>** |  | 
**player_locations** | [**Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInner>**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_plantPlayerLocations_inner.md) |  | 
**finishing_damage** | [**models::MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlayerStatsInnerKillsInnerFinishingDamage**](_match_details_v1_matches__matchID__get_200_response_roundResults_inner_playerStats_inner_kills_inner_finishingDamage.md) |  | 
**round** | **f64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


