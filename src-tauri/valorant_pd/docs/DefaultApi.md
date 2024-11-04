# \DefaultApi

All URIs are relative to *https://pd..a.pvp.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_xp_v1_players_puuid_get**](DefaultApi.md#account_xp_v1_players_puuid_get) | **GET** /account_xp/v1/players/{puuid} | Account XP
[**contract_definitions_v3_item_upgrades_get**](DefaultApi.md#contract_definitions_v3_item_upgrades_get) | **GET** /contract_definitions/v3/item_upgrades | Item Upgrades
[**contracts_v1_contracts_puuid_get**](DefaultApi.md#contracts_v1_contracts_puuid_get) | **GET** /contracts/v1/contracts/{puuid} | Contracts
[**contracts_v1_contracts_puuid_special_contract_id_post**](DefaultApi.md#contracts_v1_contracts_puuid_special_contract_id_post) | **POST** /contracts/v1/contracts/{puuid}/special/{contract_id} | Activate Contract
[**match_details_v1_matches_match_id_get**](DefaultApi.md#match_details_v1_matches_match_id_get) | **GET** /match_details/v1/matches/{matchID} | Match Details
[**match_history_v1_history_puuid_get**](DefaultApi.md#match_history_v1_history_puuid_get) | **GET** /match_history/v1/history/{puuid} | Match History
[**mmr_v1_leaderboards_affinity_na_queue_competitive_season_season_id_get**](DefaultApi.md#mmr_v1_leaderboards_affinity_na_queue_competitive_season_season_id_get) | **GET** /mmr/v1/leaderboards/affinity/na/queue/competitive/season/{season_id} | Leaderboard
[**mmr_v1_players_puuid_competitiveupdates_get**](DefaultApi.md#mmr_v1_players_puuid_competitiveupdates_get) | **GET** /mmr/v1/players/{puuid}/competitiveupdates | Competitive Updates
[**mmr_v1_players_puuid_get**](DefaultApi.md#mmr_v1_players_puuid_get) | **GET** /mmr/v1/players/{puuid} | Player MMR
[**name_service_v2_players_put**](DefaultApi.md#name_service_v2_players_put) | **PUT** /name_service/v2/players | Name Service
[**personalization_v2_players_puuid_playerloadout_get**](DefaultApi.md#personalization_v2_players_puuid_playerloadout_get) | **GET** /personalization/v2/players/{puuid}/playerloadout | Player Loadout
[**personalization_v2_players_puuid_playerloadout_put**](DefaultApi.md#personalization_v2_players_puuid_playerloadout_put) | **PUT** /personalization/v2/players/{puuid}/playerloadout | Set Player Loadout
[**restrictions_v3_penalties_get**](DefaultApi.md#restrictions_v3_penalties_get) | **GET** /restrictions/v3/penalties | Penalties
[**store_v1_entitlements_puuid_item_type_id_get**](DefaultApi.md#store_v1_entitlements_puuid_item_type_id_get) | **GET** /store/v1/entitlements/{puuid}/{ItemTypeID} | Owned Items
[**store_v1_offers_get**](DefaultApi.md#store_v1_offers_get) | **GET** /store/v1/offers/ | Prices
[**store_v1_wallet_puuid_get**](DefaultApi.md#store_v1_wallet_puuid_get) | **GET** /store/v1/wallet/{puuid} | Wallet
[**store_v2_storefront_puuid_get**](DefaultApi.md#store_v2_storefront_puuid_get) | **GET** /store/v2/storefront/{puuid} | Storefront
[**v1_config_region_get**](DefaultApi.md#v1_config_region_get) | **GET** /v1/config/{region} | Config



## account_xp_v1_players_puuid_get

> models::AccountXpV1PlayersPuuidGet200Response account_xp_v1_players_puuid_get(puuid, token, entitlement, client_platform, client_version)
Account XP

Get the account level, XP, and XP history for the current player. This endpoint only works with the authenticated player's PUUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::AccountXpV1PlayersPuuidGet200Response**](_account_xp_v1_players__puuid__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_definitions_v3_item_upgrades_get

> models::ContractDefinitionsV3ItemUpgradesGet200Response contract_definitions_v3_item_upgrades_get(token, entitlement, client_platform, client_version)
Item Upgrades

Get details for item upgrades

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::ContractDefinitionsV3ItemUpgradesGet200Response**](_contract_definitions_v3_item_upgrades_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_v1_contracts_puuid_get

> models::ContractsV1ContractsPuuidGet200Response contracts_v1_contracts_puuid_get(puuid, token, entitlement, client_version, client_platform)
Contracts

Get contract details including agents, battlepass, missions, and recent games

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_version** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |

### Return type

[**models::ContractsV1ContractsPuuidGet200Response**](_contracts_v1_contracts__puuid__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_v1_contracts_puuid_special_contract_id_post

> models::ContractsV1ContractsPuuidGet200Response contracts_v1_contracts_puuid_special_contract_id_post(contract_id, puuid, token, entitlement, client_version, client_platform)
Activate Contract

Activate a specific contract by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_version** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |

### Return type

[**models::ContractsV1ContractsPuuidGet200Response**](_contracts_v1_contracts__puuid__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## match_details_v1_matches_match_id_get

> models::MatchDetailsV1MatchesMatchIdGet200Response match_details_v1_matches_match_id_get(match_id, token, entitlement, client_version, client_platform)
Match Details

Get the details of a match after it ends

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_id** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_version** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |

### Return type

[**models::MatchDetailsV1MatchesMatchIdGet200Response**](_match_details_v1_matches__matchID__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## match_history_v1_history_puuid_get

> models::MatchHistoryV1HistoryPuuidGet200Response match_history_v1_history_puuid_get(puuid, token, entitlement, client_platform, client_version)
Match History

Get the match history for the given player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::MatchHistoryV1HistoryPuuidGet200Response**](_match_history_v1_history__puuid__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mmr_v1_leaderboards_affinity_na_queue_competitive_season_season_id_get

> models::MmrV1LeaderboardsAffinityNaQueueCompetitiveSeasonSeasonIdGet200Response mmr_v1_leaderboards_affinity_na_queue_competitive_season_season_id_get(season_id, token, entitlement, client_platform, client_version)
Leaderboard

Get the leaderboard for a given season

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**season_id** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::MmrV1LeaderboardsAffinityNaQueueCompetitiveSeasonSeasonIdGet200Response**](_mmr_v1_leaderboards_affinity_na_queue_competitive_season__season_id__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mmr_v1_players_puuid_competitiveupdates_get

> models::MmrV1PlayersPuuidCompetitiveupdatesGet200Response mmr_v1_players_puuid_competitiveupdates_get(puuid, token, entitlement, client_platform, client_version)
Competitive Updates

Get recent games and how they changed ranking

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::MmrV1PlayersPuuidCompetitiveupdatesGet200Response**](_mmr_v1_players__puuid__competitiveupdates_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mmr_v1_players_puuid_get

> models::MmrV1PlayersPuuidGet200Response mmr_v1_players_puuid_get(puuid, token, entitlement, client_platform, client_version)
Player MMR

Get a player's MMR and history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::MmrV1PlayersPuuidGet200Response**](_mmr_v1_players__puuid__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## name_service_v2_players_put

> Vec<models::NameServiceV2PlayersPut200ResponseInner> name_service_v2_players_put(token, entitlement, client_version, client_platform)
Name Service

Get a player's name and tagline by their PUUID. Supports retrieving multiple players in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_version** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |

### Return type

[**Vec<models::NameServiceV2PlayersPut200ResponseInner>**](_name_service_v2_players_put_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## personalization_v2_players_puuid_playerloadout_get

> models::PersonalizationV2PlayersPuuidPlayerloadoutGet200Response personalization_v2_players_puuid_playerloadout_get(puuid, token, entitlement, client_platform, client_version)
Player Loadout

Get the player's current loadout. Only works for your own PUUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::PersonalizationV2PlayersPuuidPlayerloadoutGet200Response**](_personalization_v2_players__puuid__playerloadout_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## personalization_v2_players_puuid_playerloadout_put

> models::PersonalizationV2PlayersPuuidPlayerloadoutGet200Response personalization_v2_players_puuid_playerloadout_put(puuid, token, entitlement, client_platform, client_version)
Set Player Loadout

Set the player's current loadout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::PersonalizationV2PlayersPuuidPlayerloadoutGet200Response**](_personalization_v2_players__puuid__playerloadout_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restrictions_v3_penalties_get

> models::RestrictionsV3PenaltiesGet200Response restrictions_v3_penalties_get(token, entitlement, client_platform, client_version)
Penalties

Get the matchmaking penalties for the given player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::RestrictionsV3PenaltiesGet200Response**](_restrictions_v3_penalties_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_v1_entitlements_puuid_item_type_id_get

> models::StoreV1EntitlementsPuuidItemTypeIdGet200Response store_v1_entitlements_puuid_item_type_id_get(item_type_id, puuid, token, entitlement, client_platform, client_version)
Owned Items

List what the player owns (agents, skins, buddies, ect.) Category names and IDs:    `ItemTypeID` | Name --- | --- `01bb38e1-da47-4e6a-9b3d-945fe4655707` | Agents `f85cb6f7-33e5-4dc8-b609-ec7212301948` | Contracts `d5f120f8-ff8c-4aac-92ea-f2b5acbe9475` | Sprays `dd3bf334-87f3-40bd-b043-682a57a8dc3a` | Gun Buddies `3f296c07-64c3-494c-923b-fe692a4fa1bd` | Cards `e7c63390-eda7-46e0-bb7a-a6abdacd2433` | Skins `3ad1b2b2-acdb-4524-852f-954a76ddae0a` | Skin Variants `de7caa6b-adf7-4588-bbd1-143831e786c6` | Titles  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_type_id** | **String** |  | [required] |
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::StoreV1EntitlementsPuuidItemTypeIdGet200Response**](_store_v1_entitlements__puuid___ItemTypeID__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_v1_offers_get

> models::StoreV1OffersGet200Response store_v1_offers_get(token, entitlement, client_platform, client_version)
Prices

Get the current store prices for all items

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::StoreV1OffersGet200Response**](_store_v1_offers__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_v1_wallet_puuid_get

> models::StoreV1WalletPuuidGet200Response store_v1_wallet_puuid_get(puuid, token, entitlement, client_platform, client_version)
Wallet

Get the current wallet balance for the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::StoreV1WalletPuuidGet200Response**](_store_v1_wallet__puuid__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_v2_storefront_puuid_get

> models::StoreV2StorefrontPuuidGet200Response store_v2_storefront_puuid_get(puuid, token, entitlement, client_platform, client_version)
Storefront

Get the currently available items in the store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**client_version** | **String** |  | [required] |

### Return type

[**models::StoreV2StorefrontPuuidGet200Response**](_store_v2_storefront__puuid__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_config_region_get

> models::V1ConfigRegionGet200Response v1_config_region_get(region)
Config

Get the config for the given player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** |  | [required] |

### Return type

[**models::V1ConfigRegionGet200Response**](_v1_config__region__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

