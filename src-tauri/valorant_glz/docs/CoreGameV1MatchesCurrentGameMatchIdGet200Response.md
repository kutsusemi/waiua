# CoreGameV1MatchesCurrentGameMatchIdGet200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**match_id** | **String** | Current Game Match ID | 
**version** | **f64** |  | 
**state** | **String** |  | 
**map_id** | **String** | Map ID | 
**mode_id** | **String** | Game Mode | 
**provisioning_flow** | **String** |  | 
**game_pod_id** | **String** |  | 
**all_muc_name** | **String** | Chat room ID for \"all\" chat | 
**team_muc_name** | **String** | Chat room ID for \"team\" chat | 
**team_voice_id** | **String** |  | 
**team_match_token** | **String** | JWT containing match ID, participant IDs, and match region | 
**is_reconnectable** | **bool** |  | 
**connection_details** | [**models::CoreGameV1MatchesCurrentGameMatchIdGet200ResponseConnectionDetails**](_core_game_v1_matches__current_game_match_id__get_200_response_ConnectionDetails.md) |  | 
**post_game_details** | Option<[**serde_json::Value**](.md)> |  | 
**players** | [**Vec<models::CoreGameV1MatchesCurrentGameMatchIdGet200ResponsePlayersInner>**](_core_game_v1_matches__current_game_match_id__get_200_response_Players_inner.md) |  | 
**matchmaking_data** | Option<[**serde_json::Value**](.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


