# PartiesV1PartiesPartyIdGet200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Party ID | 
**muc_name** | **String** |  | 
**voice_room_id** | **String** |  | 
**version** | **f64** |  | 
**client_version** | **String** |  | 
**members** | [**Vec<models::PartiesV1PartiesPartyIdGet200ResponseMembersInner>**](_parties_v1_parties__party_id__get_200_response_Members_inner.md) |  | 
**state** | **String** |  | 
**previous_state** | **String** |  | 
**state_transition_reason** | **String** |  | 
**accessibility** | [**models::PartiesV1PartiesPartyIdGet200ResponseAccessibility**](_parties_v1_parties__party_id__get_200_response_Accessibility.md) |  | 
**custom_game_data** | [**models::PartiesV1PartiesPartyIdGet200ResponseCustomGameData**](_parties_v1_parties__party_id__get_200_response_CustomGameData.md) |  | 
**matchmaking_data** | [**models::PartiesV1PartiesPartyIdGet200ResponseMatchmakingData**](_parties_v1_parties__party_id__get_200_response_MatchmakingData.md) |  | 
**invites** | Option<[**serde_json::Value**](.md)> |  | 
**requests** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**queue_entry_time** | **String** | Date in ISO 8601 format | 
**error_notification** | [**models::PartiesV1PartiesPartyIdGet200ResponseErrorNotification**](_parties_v1_parties__party_id__get_200_response_ErrorNotification.md) |  | 
**restricted_seconds** | **f64** |  | 
**eligible_queues** | **Vec<String>** |  | 
**queue_ineligibilities** | **Vec<String>** |  | 
**cheat_data** | [**models::PartiesV1PartiesPartyIdGet200ResponseCheatData**](_parties_v1_parties__party_id__get_200_response_CheatData.md) |  | 
**xp_bonuses** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**invite_code** | **String** | Empty string when there is no invite code | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


