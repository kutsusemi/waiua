# Rust API client for valorant_glz

Valorant API


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Generator version: 7.9.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `valorant_glz` and add the following to `Cargo.toml` under `[dependencies]`:

```
valorant_glz = { path = "./valorant_glz" }
```

## Documentation for API Endpoints

All URIs are relative to *https://glz--1..a.pvp.net*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**core_game_v1_matches_current_game_match_id_get**](docs/DefaultApi.md#core_game_v1_matches_current_game_match_id_get) | **GET** /core-game/v1/matches/{current_game_match_id} | Current Game Match
*DefaultApi* | [**core_game_v1_matches_current_game_match_id_loadouts_get**](docs/DefaultApi.md#core_game_v1_matches_current_game_match_id_loadouts_get) | **GET** /core-game/v1/matches/{current_game_match_id}/loadouts | Current Game Loadouts
*DefaultApi* | [**core_game_v1_players_puuid_disassociate_current_game_match_id_post**](docs/DefaultApi.md#core_game_v1_players_puuid_disassociate_current_game_match_id_post) | **POST** /core-game/v1/players/{puuid}/disassociate/{current_game_match_id} | Current Game Quit
*DefaultApi* | [**core_game_v1_players_puuid_get**](docs/DefaultApi.md#core_game_v1_players_puuid_get) | **GET** /core-game/v1/players/{puuid} | Current Game Player
*DefaultApi* | [**parties_v1_parties_customgameconfigs_get**](docs/DefaultApi.md#parties_v1_parties_customgameconfigs_get) | **GET** /parties/v1/parties/customgameconfigs | Custom Game Configs
*DefaultApi* | [**parties_v1_parties_party_id_accessibility_post**](docs/DefaultApi.md#parties_v1_parties_party_id_accessibility_post) | **POST** /parties/v1/parties/{party_id}/accessibility | Set Party Accessibility
*DefaultApi* | [**parties_v1_parties_party_id_customgamesettings_post**](docs/DefaultApi.md#parties_v1_parties_party_id_customgamesettings_post) | **POST** /parties/v1/parties/{party_id}/customgamesettings | Set Custom Game Settings
*DefaultApi* | [**parties_v1_parties_party_id_get**](docs/DefaultApi.md#parties_v1_parties_party_id_get) | **GET** /parties/v1/parties/{party_id} | Party
*DefaultApi* | [**parties_v1_parties_party_id_invitecode_delete**](docs/DefaultApi.md#parties_v1_parties_party_id_invitecode_delete) | **DELETE** /parties/v1/parties/{party_id}/invitecode | Party Disable Code
*DefaultApi* | [**parties_v1_parties_party_id_invitecode_post**](docs/DefaultApi.md#parties_v1_parties_party_id_invitecode_post) | **POST** /parties/v1/parties/{party_id}/invitecode | Party Generate Code
*DefaultApi* | [**parties_v1_parties_party_id_invites_name_name_tag_tagline_post**](docs/DefaultApi.md#parties_v1_parties_party_id_invites_name_name_tag_tagline_post) | **POST** /parties/v1/parties/{party_id}/invites/name/{name}/tag/{tagline} | Party Invite
*DefaultApi* | [**parties_v1_parties_party_id_matchmaking_join_post**](docs/DefaultApi.md#parties_v1_parties_party_id_matchmaking_join_post) | **POST** /parties/v1/parties/{party_id}/matchmaking/join | Enter Matchmaking Queue
*DefaultApi* | [**parties_v1_parties_party_id_matchmaking_leave_post**](docs/DefaultApi.md#parties_v1_parties_party_id_matchmaking_leave_post) | **POST** /parties/v1/parties/{party_id}/matchmaking/leave | Leave Matchmaking Queue
*DefaultApi* | [**parties_v1_parties_party_id_members_puuid_refresh_competitive_tier_post**](docs/DefaultApi.md#parties_v1_parties_party_id_members_puuid_refresh_competitive_tier_post) | **POST** /parties/v1/parties/{party_id}/members/{puuid}/refreshCompetitiveTier | Refresh Competitive Tier
*DefaultApi* | [**parties_v1_parties_party_id_members_puuid_refresh_pings_post**](docs/DefaultApi.md#parties_v1_parties_party_id_members_puuid_refresh_pings_post) | **POST** /parties/v1/parties/{party_id}/members/{puuid}/refreshPings | Refresh Pings
*DefaultApi* | [**parties_v1_parties_party_id_members_puuid_refresh_player_identity_post**](docs/DefaultApi.md#parties_v1_parties_party_id_members_puuid_refresh_player_identity_post) | **POST** /parties/v1/parties/{party_id}/members/{puuid}/refreshPlayerIdentity | Refresh Player Identity
*DefaultApi* | [**parties_v1_parties_party_id_members_puuid_set_ready_post**](docs/DefaultApi.md#parties_v1_parties_party_id_members_puuid_set_ready_post) | **POST** /parties/v1/parties/{party_id}/members/{puuid}/setReady | Party Set Member Ready
*DefaultApi* | [**parties_v1_parties_party_id_muctoken_get**](docs/DefaultApi.md#parties_v1_parties_party_id_muctoken_get) | **GET** //parties/v1/parties/{party_id}/muctoken | Party Chat Token
*DefaultApi* | [**parties_v1_parties_party_id_queue_post**](docs/DefaultApi.md#parties_v1_parties_party_id_queue_post) | **POST** /parties/v1/parties/{party_id}/queue | Change Queue
*DefaultApi* | [**parties_v1_parties_party_id_request_post**](docs/DefaultApi.md#parties_v1_parties_party_id_request_post) | **POST** /parties/v1/parties/{party_id}/request | Party Request
*DefaultApi* | [**parties_v1_parties_party_id_request_request_id_decline_post**](docs/DefaultApi.md#parties_v1_parties_party_id_request_request_id_decline_post) | **POST** /parties/v1/parties/{party_id}/request/{request_id}/decline | Party Decline
*DefaultApi* | [**parties_v1_parties_party_id_startcustomgame_post**](docs/DefaultApi.md#parties_v1_parties_party_id_startcustomgame_post) | **POST** /parties/v1/parties/{party_id}/startcustomgame | Start Custom Game
*DefaultApi* | [**parties_v1_parties_party_id_voicetoken_get**](docs/DefaultApi.md#parties_v1_parties_party_id_voicetoken_get) | **GET** //parties/v1/parties/{party_id}/voicetoken | Party Voice Token
*DefaultApi* | [**parties_v1_players_joinbycode_code_post**](docs/DefaultApi.md#parties_v1_players_joinbycode_code_post) | **POST** /parties/v1/players/joinbycode/{code} | Party Join By Code
*DefaultApi* | [**parties_v1_players_puuid_delete**](docs/DefaultApi.md#parties_v1_players_puuid_delete) | **DELETE** /parties/v1/players/{puuid} | Party Remove Player
*DefaultApi* | [**parties_v1_players_puuid_get**](docs/DefaultApi.md#parties_v1_players_puuid_get) | **GET** /parties/v1/players/{puuid} | Party Player
*DefaultApi* | [**pregame_v1_matches_pre_game_match_id_get**](docs/DefaultApi.md#pregame_v1_matches_pre_game_match_id_get) | **GET** /pregame/v1/matches/{pre_game_match_id} | Pre-Game Match
*DefaultApi* | [**pregame_v1_matches_pre_game_match_id_loadouts_get**](docs/DefaultApi.md#pregame_v1_matches_pre_game_match_id_loadouts_get) | **GET** /pregame/v1/matches/{pre_game_match_id}/loadouts | Pre-Game Loadouts
*DefaultApi* | [**pregame_v1_matches_pre_game_match_id_lock_agent_id_post**](docs/DefaultApi.md#pregame_v1_matches_pre_game_match_id_lock_agent_id_post) | **POST** /pregame/v1/matches/{pre_game_match_id}/lock/{agent_id} | Lock Character
*DefaultApi* | [**pregame_v1_matches_pre_game_match_id_quit_post**](docs/DefaultApi.md#pregame_v1_matches_pre_game_match_id_quit_post) | **POST** /pregame/v1/matches/{pre_game_match_id}/quit | Pre-Game Quit
*DefaultApi* | [**pregame_v1_matches_pre_game_match_id_select_agent_id_post**](docs/DefaultApi.md#pregame_v1_matches_pre_game_match_id_select_agent_id_post) | **POST** /pregame/v1/matches/{pre_game_match_id}/select/{agent_id} | Select Character
*DefaultApi* | [**pregame_v1_players_puuid_get**](docs/DefaultApi.md#pregame_v1_players_puuid_get) | **GET** /pregame/v1/players/{puuid} | Pre-Game Player


## Documentation For Models

 - [CoreGameV1MatchesCurrentGameMatchIdGet200Response](docs/CoreGameV1MatchesCurrentGameMatchIdGet200Response.md)
 - [CoreGameV1MatchesCurrentGameMatchIdGet200ResponseConnectionDetails](docs/CoreGameV1MatchesCurrentGameMatchIdGet200ResponseConnectionDetails.md)
 - [CoreGameV1MatchesCurrentGameMatchIdGet200ResponsePlayersInner](docs/CoreGameV1MatchesCurrentGameMatchIdGet200ResponsePlayersInner.md)
 - [CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200Response](docs/CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200Response.md)
 - [CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200ResponseLoadoutsInner](docs/CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200ResponseLoadoutsInner.md)
 - [PartiesV1PartiesCustomgameconfigsGet200Response](docs/PartiesV1PartiesCustomgameconfigsGet200Response.md)
 - [PartiesV1PartiesCustomgameconfigsGet200ResponseGamePodPingServiceInfoValue](docs/PartiesV1PartiesCustomgameconfigsGet200ResponseGamePodPingServiceInfoValue.md)
 - [PartiesV1PartiesCustomgameconfigsGet200ResponseQueuesInner](docs/PartiesV1PartiesCustomgameconfigsGet200ResponseQueuesInner.md)
 - [PartiesV1PartiesPartyIdGet200Response](docs/PartiesV1PartiesPartyIdGet200Response.md)
 - [PartiesV1PartiesPartyIdGet200ResponseAccessibility](docs/PartiesV1PartiesPartyIdGet200ResponseAccessibility.md)
 - [PartiesV1PartiesPartyIdGet200ResponseCheatData](docs/PartiesV1PartiesPartyIdGet200ResponseCheatData.md)
 - [PartiesV1PartiesPartyIdGet200ResponseCustomGameData](docs/PartiesV1PartiesPartyIdGet200ResponseCustomGameData.md)
 - [PartiesV1PartiesPartyIdGet200ResponseCustomGameDataMembership](docs/PartiesV1PartiesPartyIdGet200ResponseCustomGameDataMembership.md)
 - [PartiesV1PartiesPartyIdGet200ResponseCustomGameDataMembershipTeamOneInner](docs/PartiesV1PartiesPartyIdGet200ResponseCustomGameDataMembershipTeamOneInner.md)
 - [PartiesV1PartiesPartyIdGet200ResponseCustomGameDataSettings](docs/PartiesV1PartiesPartyIdGet200ResponseCustomGameDataSettings.md)
 - [PartiesV1PartiesPartyIdGet200ResponseCustomGameDataSettingsGameRules](docs/PartiesV1PartiesPartyIdGet200ResponseCustomGameDataSettingsGameRules.md)
 - [PartiesV1PartiesPartyIdGet200ResponseErrorNotification](docs/PartiesV1PartiesPartyIdGet200ResponseErrorNotification.md)
 - [PartiesV1PartiesPartyIdGet200ResponseMatchmakingData](docs/PartiesV1PartiesPartyIdGet200ResponseMatchmakingData.md)
 - [PartiesV1PartiesPartyIdGet200ResponseMembersInner](docs/PartiesV1PartiesPartyIdGet200ResponseMembersInner.md)
 - [PartiesV1PartiesPartyIdGet200ResponseMembersInnerPingsInner](docs/PartiesV1PartiesPartyIdGet200ResponseMembersInnerPingsInner.md)
 - [PartiesV1PartiesPartyIdGet200ResponseMembersInnerPlayerIdentity](docs/PartiesV1PartiesPartyIdGet200ResponseMembersInnerPlayerIdentity.md)
 - [PartiesV1PartiesPartyIdGet200ResponseMembersInnerPlayerIdentityPreferredLevelBorderId](docs/PartiesV1PartiesPartyIdGet200ResponseMembersInnerPlayerIdentityPreferredLevelBorderId.md)
 - [PartiesV1PartiesPartyIdMuctokenGet200Response](docs/PartiesV1PartiesPartyIdMuctokenGet200Response.md)
 - [PartiesV1PlayersJoinbycodeCodePost404Response](docs/PartiesV1PlayersJoinbycodeCodePost404Response.md)
 - [PartiesV1PlayersPuuidGet200Response](docs/PartiesV1PlayersPuuidGet200Response.md)
 - [PartiesV1PlayersPuuidGet200ResponsePlatformInfo](docs/PartiesV1PlayersPuuidGet200ResponsePlatformInfo.md)
 - [PartiesV1PlayersPuuidGet200ResponseRequestsInner](docs/PartiesV1PlayersPuuidGet200ResponseRequestsInner.md)
 - [PregameV1MatchesPreGameMatchIdGet200Response](docs/PregameV1MatchesPreGameMatchIdGet200Response.md)
 - [PregameV1MatchesPreGameMatchIdGet200ResponseAllyTeam](docs/PregameV1MatchesPreGameMatchIdGet200ResponseAllyTeam.md)
 - [PregameV1MatchesPreGameMatchIdGet200ResponseQueueId](docs/PregameV1MatchesPreGameMatchIdGet200ResponseQueueId.md)
 - [PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInner](docs/PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInner.md)
 - [PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInner](docs/PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInner.md)
 - [PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInnerSeasonalBadgeInfo](docs/PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInnerSeasonalBadgeInfo.md)
 - [PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInnerSeasonalBadgeInfoSeasonId](docs/PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerPlayersInnerSeasonalBadgeInfoSeasonId.md)
 - [PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerTeamId](docs/PregameV1MatchesPreGameMatchIdGet200ResponseTeamsInnerTeamId.md)
 - [PregameV1MatchesPreGameMatchIdLoadoutsGet200Response](docs/PregameV1MatchesPreGameMatchIdLoadoutsGet200Response.md)
 - [PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInner](docs/PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInner.md)
 - [PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerExpressions](docs/PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerExpressions.md)
 - [PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerExpressionsAesSelectionsInner](docs/PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerExpressionsAesSelectionsInner.md)
 - [PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerItemsValue](docs/PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerItemsValue.md)
 - [PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerItemsValueSocketsValue](docs/PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerItemsValueSocketsValue.md)
 - [PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerItemsValueSocketsValueItem](docs/PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerItemsValueSocketsValueItem.md)
 - [PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerSprays](docs/PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerSprays.md)
 - [PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerSpraysSpraySelectionsInner](docs/PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerSpraysSpraySelectionsInner.md)
 - [PregameV1PlayersPuuidGet200Response](docs/PregameV1PlayersPuuidGet200Response.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



