# PartiesV1PartiesCustomgameconfigsGet200ResponseQueuesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | 
**enabled** | **bool** |  | 
**team_size** | **f64** |  | 
**num_teams** | **f64** |  | 
**max_party_size** | **f64** |  | 
**min_party_size** | **f64** |  | 
**invalid_party_sizes** | **Vec<f64>** |  | 
**max_party_size_high_skill** | **f64** |  | 
**high_skill_tier** | **f64** |  | 
**max_skill_tier** | **f64** |  | 
**allow_full_party_bypass_skill_restrictions** | **bool** |  | 
**mode** | **String** |  | 
**is_ranked** | **bool** |  | 
**is_tournament** | **bool** |  | 
**require_roster** | **bool** |  | 
**priority** | **f64** |  | 
**party_max_competitive_tier_range** | **f64** |  | 
**party_max_competitive_tier_range_placement_buffer** | **f64** |  | 
**full_party_max_competitive_tier_range** | **f64** |  | 
**party_skill_disparity_competitive_tiers_ceilings** | **std::collections::HashMap<String, f64>** |  | 
**use_account_level_requirement** | **bool** |  | 
**minimum_account_level_required** | **f64** |  | 
**game_rules** | **std::collections::HashMap<String, String>** |  | 
**supported_platform_types** | **Vec<String>** |  | 
**disabled_content** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**queue_field_a** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**next_schedule_change_seconds** | **f64** |  | 
**time_until_next_schedule_change_seconds** | **f64** |  | 
**map_weights** | **Vec<String>** | Array of strings in the format of \"map:weight\" | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


