# \DefaultApi

All URIs are relative to *https://glz--1..a.pvp.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**core_game_v1_matches_current_game_match_id_get**](DefaultApi.md#core_game_v1_matches_current_game_match_id_get) | **GET** /core_game/v1/matches/{current_game_match_id} | Current Game Match
[**core_game_v1_matches_current_game_match_id_loadouts_get**](DefaultApi.md#core_game_v1_matches_current_game_match_id_loadouts_get) | **GET** /core_game/v1/matches/{current_game_match_id}/loadouts | Current Game Loadouts
[**core_game_v1_players_puuid_disassociate_current_game_match_id_post**](DefaultApi.md#core_game_v1_players_puuid_disassociate_current_game_match_id_post) | **POST** /core_game/v1/players/{puuid}/disassociate/{current_game_match_id} | Current Game Quit
[**core_game_v1_players_puuid_get**](DefaultApi.md#core_game_v1_players_puuid_get) | **GET** /core_game/v1/players/{puuid} | Current Game Player
[**parties_v1_parties_customgameconfigs_get**](DefaultApi.md#parties_v1_parties_customgameconfigs_get) | **GET** /parties/v1/parties/customgameconfigs | Custom Game Configs
[**parties_v1_parties_party_id_accessibility_post**](DefaultApi.md#parties_v1_parties_party_id_accessibility_post) | **POST** /parties/v1/parties/{party_id}/accessibility | Set Party Accessibility
[**parties_v1_parties_party_id_customgamesettings_post**](DefaultApi.md#parties_v1_parties_party_id_customgamesettings_post) | **POST** /parties/v1/parties/{party_id}/customgamesettings | Set Custom Game Settings
[**parties_v1_parties_party_id_get**](DefaultApi.md#parties_v1_parties_party_id_get) | **GET** /parties/v1/parties/{party_id} | Party
[**parties_v1_parties_party_id_invitecode_delete**](DefaultApi.md#parties_v1_parties_party_id_invitecode_delete) | **DELETE** /parties/v1/parties/{party_id}/invitecode | Party Disable Code
[**parties_v1_parties_party_id_invitecode_post**](DefaultApi.md#parties_v1_parties_party_id_invitecode_post) | **POST** /parties/v1/parties/{party_id}/invitecode | Party Generate Code
[**parties_v1_parties_party_id_invites_name_name_tag_tagline_post**](DefaultApi.md#parties_v1_parties_party_id_invites_name_name_tag_tagline_post) | **POST** /parties/v1/parties/{party_id}/invites/name/{name}/tag/{tagline} | Party Invite
[**parties_v1_parties_party_id_matchmaking_join_post**](DefaultApi.md#parties_v1_parties_party_id_matchmaking_join_post) | **POST** /parties/v1/parties/{party_id}/matchmaking/join | Enter Matchmaking Queue
[**parties_v1_parties_party_id_matchmaking_leave_post**](DefaultApi.md#parties_v1_parties_party_id_matchmaking_leave_post) | **POST** /parties/v1/parties/{party_id}/matchmaking/leave | Leave Matchmaking Queue
[**parties_v1_parties_party_id_members_puuid_refresh_competitive_tier_post**](DefaultApi.md#parties_v1_parties_party_id_members_puuid_refresh_competitive_tier_post) | **POST** /parties/v1/parties/{party_id}/members/{puuid}/refreshCompetitiveTier | Refresh Competitive Tier
[**parties_v1_parties_party_id_members_puuid_refresh_pings_post**](DefaultApi.md#parties_v1_parties_party_id_members_puuid_refresh_pings_post) | **POST** /parties/v1/parties/{party_id}/members/{puuid}/refreshPings | Refresh Pings
[**parties_v1_parties_party_id_members_puuid_refresh_player_identity_post**](DefaultApi.md#parties_v1_parties_party_id_members_puuid_refresh_player_identity_post) | **POST** /parties/v1/parties/{party_id}/members/{puuid}/refreshPlayerIdentity | Refresh Player Identity
[**parties_v1_parties_party_id_members_puuid_set_ready_post**](DefaultApi.md#parties_v1_parties_party_id_members_puuid_set_ready_post) | **POST** /parties/v1/parties/{party_id}/members/{puuid}/setReady | Party Set Member Ready
[**parties_v1_parties_party_id_muctoken_get**](DefaultApi.md#parties_v1_parties_party_id_muctoken_get) | **GET** //parties/v1/parties/{party_id}/muctoken | Party Chat Token
[**parties_v1_parties_party_id_queue_post**](DefaultApi.md#parties_v1_parties_party_id_queue_post) | **POST** /parties/v1/parties/{party_id}/queue | Change Queue
[**parties_v1_parties_party_id_request_post**](DefaultApi.md#parties_v1_parties_party_id_request_post) | **POST** /parties/v1/parties/{party_id}/request | Party Request
[**parties_v1_parties_party_id_request_request_id_decline_post**](DefaultApi.md#parties_v1_parties_party_id_request_request_id_decline_post) | **POST** /parties/v1/parties/{party_id}/request/{request_id}/decline | Party Decline
[**parties_v1_parties_party_id_startcustomgame_post**](DefaultApi.md#parties_v1_parties_party_id_startcustomgame_post) | **POST** /parties/v1/parties/{party_id}/startcustomgame | Start Custom Game
[**parties_v1_parties_party_id_voicetoken_get**](DefaultApi.md#parties_v1_parties_party_id_voicetoken_get) | **GET** //parties/v1/parties/{party_id}/voicetoken | Party Voice Token
[**parties_v1_players_joinbycode_code_post**](DefaultApi.md#parties_v1_players_joinbycode_code_post) | **POST** /parties/v1/players/joinbycode/{code} | Party Join By Code
[**parties_v1_players_puuid_delete**](DefaultApi.md#parties_v1_players_puuid_delete) | **DELETE** /parties/v1/players/{puuid} | Party Remove Player
[**parties_v1_players_puuid_get**](DefaultApi.md#parties_v1_players_puuid_get) | **GET** /parties/v1/players/{puuid} | Party Player
[**pregame_v1_matches_pre_game_match_id_get**](DefaultApi.md#pregame_v1_matches_pre_game_match_id_get) | **GET** /pregame/v1/matches/{pre_game_match_id} | Pre-Game Match
[**pregame_v1_matches_pre_game_match_id_loadouts_get**](DefaultApi.md#pregame_v1_matches_pre_game_match_id_loadouts_get) | **GET** /pregame/v1/matches/{pre_game_match_id}/loadouts | Pre-Game Loadouts
[**pregame_v1_matches_pre_game_match_id_lock_agent_id_post**](DefaultApi.md#pregame_v1_matches_pre_game_match_id_lock_agent_id_post) | **POST** /pregame/v1/matches/{pre_game_match_id}/lock/{agent_id} | Lock Character
[**pregame_v1_matches_pre_game_match_id_quit_post**](DefaultApi.md#pregame_v1_matches_pre_game_match_id_quit_post) | **POST** /pregame/v1/matches/{pre_game_match_id}/quit | Pre-Game Quit
[**pregame_v1_matches_pre_game_match_id_select_agent_id_post**](DefaultApi.md#pregame_v1_matches_pre_game_match_id_select_agent_id_post) | **POST** /pregame/v1/matches/{pre_game_match_id}/select/{agent_id} | Select Character
[**pregame_v1_players_puuid_get**](DefaultApi.md#pregame_v1_players_puuid_get) | **GET** /pregame/v1/players/{puuid} | Pre-Game Player



## core_game_v1_matches_current_game_match_id_get

> models::CoreGameV1MatchesCurrentGameMatchIdGet200Response core_game_v1_matches_current_game_match_id_get(current_game_match_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Current Game Match

Get the current game match info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**current_game_match_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::CoreGameV1MatchesCurrentGameMatchIdGet200Response**](_core_game_v1_matches__current_game_match_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_game_v1_matches_current_game_match_id_loadouts_get

> models::CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200Response core_game_v1_matches_current_game_match_id_loadouts_get(current_game_match_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Current Game Loadouts

Get the current game loadout info for all players in the match

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**current_game_match_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200Response**](_core_game_v1_matches__current_game_match_id__loadouts_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_game_v1_players_puuid_disassociate_current_game_match_id_post

> serde_json::Value core_game_v1_players_puuid_disassociate_current_game_match_id_post(puuid, current_game_match_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Current Game Quit

Quits the current game

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**current_game_match_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_game_v1_players_puuid_get

> models::PregameV1PlayersPuuidGet200Response core_game_v1_players_puuid_get(puuid, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Current Game Player

Get the current game match ID for the provided player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PregameV1PlayersPuuidGet200Response**](_pregame_v1_players__puuid__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_customgameconfigs_get

> models::PartiesV1PartiesCustomgameconfigsGet200Response parties_v1_parties_customgameconfigs_get(x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Custom Game Configs

Get information about the available gamemodes, maps, queues, and gamepods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesCustomgameconfigsGet200Response**](_parties_v1_parties_customgameconfigs_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_accessibility_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_accessibility_post(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Set Party Accessibility

Set the accessibility of the party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_customgamesettings_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_customgamesettings_post(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Set Custom Game Settings

Changes the settings for a custom game

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_get

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_get(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party

Get the party information for the given party ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_invitecode_delete

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_invitecode_delete(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Disable Code

Disable the party invite code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_invitecode_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_invitecode_post(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Generate Code

Generate a party invite code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_invites_name_name_tag_tagline_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_invites_name_name_tag_tagline_post(name, tagline, party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Invite

Invite a player to the party by name and tagline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**tagline** | **String** |  | [required] |
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_matchmaking_join_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_matchmaking_join_post(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Enter Matchmaking Queue

Enter the matchmaking queue for the party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_matchmaking_leave_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_matchmaking_leave_post(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Leave Matchmaking Queue

Leave the matchmaking queue for the party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_members_puuid_refresh_competitive_tier_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_members_puuid_refresh_competitive_tier_post(party_id, puuid, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Refresh Competitive Tier

Refresh the competitive tier of the specified player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**puuid** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_members_puuid_refresh_pings_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_members_puuid_refresh_pings_post(party_id, puuid, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Refresh Pings

Refresh the pings of the specified player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**puuid** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_members_puuid_refresh_player_identity_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_members_puuid_refresh_player_identity_post(party_id, puuid, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Refresh Player Identity

Refresh the identity of the specified player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**puuid** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_members_puuid_set_ready_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_members_puuid_set_ready_post(party_id, puuid, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Set Member Ready

Set the ready status of a player in the current party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**puuid** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_muctoken_get

> models::PartiesV1PartiesPartyIdMuctokenGet200Response parties_v1_parties_party_id_muctoken_get(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Chat Token

Get the party chat token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdMuctokenGet200Response**](__parties_v1_parties__party_id__muctoken_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_queue_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_queue_post(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Change Queue

Change the queue for the party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_request_post

> serde_json::Value parties_v1_parties_party_id_request_post(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Request

Requests to join the specified party ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_request_request_id_decline_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_request_request_id_decline_post(request_id, party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Decline

Decline a party invite request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_startcustomgame_post

> models::PartiesV1PartiesPartyIdGet200Response parties_v1_parties_party_id_startcustomgame_post(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Start Custom Game

Start a custom game

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdGet200Response**](_parties_v1_parties__party_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_parties_party_id_voicetoken_get

> models::PartiesV1PartiesPartyIdMuctokenGet200Response parties_v1_parties_party_id_voicetoken_get(party_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Voice Token

Get the party voice token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**party_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PartiesPartyIdMuctokenGet200Response**](__parties_v1_parties__party_id__muctoken_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_players_joinbycode_code_post

> models::PartiesV1PlayersPuuidGet200Response parties_v1_players_joinbycode_code_post(code, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Join By Code

Join a party using an invite code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PlayersPuuidGet200Response**](_parties_v1_players__puuid__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_players_puuid_delete

> serde_json::Value parties_v1_players_puuid_delete(puuid, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Remove Player

Remove a player from the current party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parties_v1_players_puuid_get

> models::PartiesV1PlayersPuuidGet200Response parties_v1_players_puuid_get(puuid, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Party Player

Get the party information for the given player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PartiesV1PlayersPuuidGet200Response**](_parties_v1_players__puuid__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pregame_v1_matches_pre_game_match_id_get

> models::PregameV1MatchesPreGameMatchIdGet200Response pregame_v1_matches_pre_game_match_id_get(pre_game_match_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Pre-Game Match

Get Pre-Game match data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pre_game_match_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PregameV1MatchesPreGameMatchIdGet200Response**](_pregame_v1_matches__pre_game_match_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pregame_v1_matches_pre_game_match_id_loadouts_get

> models::PregameV1MatchesPreGameMatchIdLoadoutsGet200Response pregame_v1_matches_pre_game_match_id_loadouts_get(pre_game_match_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Pre-Game Loadouts

Get Pre-Game loadout data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pre_game_match_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PregameV1MatchesPreGameMatchIdLoadoutsGet200Response**](_pregame_v1_matches__pre_game_match_id__loadouts_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pregame_v1_matches_pre_game_match_id_lock_agent_id_post

> models::PregameV1MatchesPreGameMatchIdGet200Response pregame_v1_matches_pre_game_match_id_lock_agent_id_post(agent_id, pre_game_match_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Lock Character

Lock in an agent   **DO NOT USE THIS FOR INSTALOCKING**   Riot doesn't like this. You may get banned or get the API restricted for the rest of us.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** |  | [required] |
**pre_game_match_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PregameV1MatchesPreGameMatchIdGet200Response**](_pregame_v1_matches__pre_game_match_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pregame_v1_matches_pre_game_match_id_quit_post

> serde_json::Value pregame_v1_matches_pre_game_match_id_quit_post(pre_game_match_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Pre-Game Quit

Quit the pre-game lobby

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pre_game_match_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pregame_v1_matches_pre_game_match_id_select_agent_id_post

> models::PregameV1MatchesPreGameMatchIdGet200Response pregame_v1_matches_pre_game_match_id_select_agent_id_post(agent_id, pre_game_match_id, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Select Character

Select an agent   **DO NOT USE THIS FOR INSTALOCKING**   Riot doesn't like this. You may get banned or get the API restricted for the rest of us.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** |  | [required] |
**pre_game_match_id** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PregameV1MatchesPreGameMatchIdGet200Response**](_pregame_v1_matches__pre_game_match_id__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pregame_v1_players_puuid_get

> models::PregameV1PlayersPuuidGet200Response pregame_v1_players_puuid_get(puuid, x_riot_entitlements_jwt, x_riot_client_version, x_riot_client_platform)
Pre-Game Player

Get the pre-game match ID for the provided player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**x_riot_entitlements_jwt** | **String** |  | [required] |
**x_riot_client_version** | **String** |  | [required] |
**x_riot_client_platform** | **String** |  | [required] |

### Return type

[**models::PregameV1PlayersPuuidGet200Response**](_pregame_v1_players__puuid__get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

