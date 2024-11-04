# PregameV1MatchesPreGameMatchIdGet200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Pre-Game Match ID | 
**version** | **f64** |  | 
**teams** | [**Vec<models::PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInner>**](_pregame_v1_matches__pre_game_match_id__get_200_response_Teams_inner.md) |  | 
**ally_team** | Option<[**models::PregameV1MatchesPreGameMatchIdGet200ResponseAllyTeam**](_pregame_v1_matches__pre_game_match_id__get_200_response_AllyTeam.md)> |  | 
**enemy_team** | Option<[**models::PregameV1MatchesPreGameMatchIdGet200ResponseAllyTeam**](_pregame_v1_matches__pre_game_match_id__get_200_response_AllyTeam.md)> |  | 
**observer_subjects** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**match_coaches** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**enemy_team_size** | **f64** |  | 
**enemy_team_lock_count** | **f64** |  | 
**pregame_state** | **String** |  | 
**last_updated** | **String** | Date in ISO 8601 format | 
**map_id** | **String** | Map ID | 
**map_select_pool** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**banned_map_ids** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**casted_votes** | Option<[**serde_json::Value**](.md)> |  | [optional]
**map_select_steps** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**map_select_step** | **f64** |  | 
**team1** | [**models::PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerTeamId**](_pregame_v1_matches__pre_game_match_id__get_200_response_Teams_inner_TeamID.md) |  | 
**game_pod_id** | **String** |  | 
**mode** | **String** | Game Mode | 
**voice_session_id** | **String** |  | 
**muc_name** | **String** |  | 
**team_match_token** | **String** | JWT containing match ID and player IDs | 
**queue_id** | [**models::PregameV1MatchesPreGameMatchIdGet200ResponseQueueId**](_pregame_v1_matches__pre_game_match_id__get_200_response_QueueID.md) |  | 
**provisioning_flow_id** | **String** |  | 
**is_ranked** | **bool** |  | 
**phase_time_remaining_ns** | **f64** |  | 
**step_time_remaining_ns** | **f64** |  | 
**alt_modes_flag_ada** | **bool** |  | 
**tournament_metadata** | Option<[**serde_json::Value**](.md)> |  | 
**roster_metadata** | Option<[**serde_json::Value**](.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


