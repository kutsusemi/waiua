# Rust API client for valorant_local

Valorant API


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Generator version: 7.9.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `valorant_local` and add the following to `Cargo.toml` under `[dependencies]`:

```
valorant_local = { path = "./valorant_local" }
```

## Documentation for API Endpoints

All URIs are relative to *https://127.0.0.1:*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**chat_v1_session_get**](docs/DefaultApi.md#chat_v1_session_get) | **GET** /chat/v1/session | Chat Session
*DefaultApi* | [**chat_v4_friendrequests_delete**](docs/DefaultApi.md#chat_v4_friendrequests_delete) | **DELETE** /chat/v4/friendrequests | Remove Friend Request
*DefaultApi* | [**chat_v4_friendrequests_get**](docs/DefaultApi.md#chat_v4_friendrequests_get) | **GET** /chat/v4/friendrequests | Friend Requests
*DefaultApi* | [**chat_v4_friendrequests_post**](docs/DefaultApi.md#chat_v4_friendrequests_post) | **POST** /chat/v4/friendrequests | Send Friend Request
*DefaultApi* | [**chat_v4_friends_get**](docs/DefaultApi.md#chat_v4_friends_get) | **GET** /chat/v4/friends | Friends
*DefaultApi* | [**chat_v4_presences_get**](docs/DefaultApi.md#chat_v4_presences_get) | **GET** /chat/v4/presences | Presence
*DefaultApi* | [**chat_v5_participants_get**](docs/DefaultApi.md#chat_v5_participants_get) | **GET** /chat/v5/participants | Chat Participants
*DefaultApi* | [**chat_v6_conversations_ares_coregame_get**](docs/DefaultApi.md#chat_v6_conversations_ares_coregame_get) | **GET** /chat/v6/conversations/ares-coregame | Current Game Chat Info
*DefaultApi* | [**chat_v6_conversations_ares_parties_get**](docs/DefaultApi.md#chat_v6_conversations_ares_parties_get) | **GET** /chat/v6/conversations/ares-parties | Party Chat Info
*DefaultApi* | [**chat_v6_conversations_ares_pregame_get**](docs/DefaultApi.md#chat_v6_conversations_ares_pregame_get) | **GET** /chat/v6/conversations/ares-pregame | Pre-Game Chat Info
*DefaultApi* | [**chat_v6_conversations_get**](docs/DefaultApi.md#chat_v6_conversations_get) | **GET** /chat/v6/conversations | All Chat Info
*DefaultApi* | [**chat_v6_messages_get**](docs/DefaultApi.md#chat_v6_messages_get) | **GET** /chat/v6/messages | Chat History
*DefaultApi* | [**chat_v6_messages_post**](docs/DefaultApi.md#chat_v6_messages_post) | **POST** /chat/v6/messages | Send Chat
*DefaultApi* | [**entitlements_v1_token_get**](docs/DefaultApi.md#entitlements_v1_token_get) | **GET** /entitlements/v1/token | Entitlements Token
*DefaultApi* | [**help_get**](docs/DefaultApi.md#help_get) | **GET** /help | Local Help
*DefaultApi* | [**player_account_aliases_v1_active_get**](docs/DefaultApi.md#player_account_aliases_v1_active_get) | **GET** /player-account/aliases/v1/active | Account Alias
*DefaultApi* | [**product_session_v1_external_sessions_get**](docs/DefaultApi.md#product_session_v1_external_sessions_get) | **GET** /product-session/v1/external-sessions | Sessions
*DefaultApi* | [**riotclient_region_locale_get**](docs/DefaultApi.md#riotclient_region_locale_get) | **GET** /riotclient/region-locale | Client Region
*DefaultApi* | [**rso_auth_v1_authorization_userinfo_get**](docs/DefaultApi.md#rso_auth_v1_authorization_userinfo_get) | **GET** /rso-auth/v1/authorization/userinfo | RSO User Info
*DefaultApi* | [**swagger_v3_openapi_json_get**](docs/DefaultApi.md#swagger_v3_openapi_json_get) | **GET** /swagger/v3/openapi.json | Local Swagger Docs


## Documentation For Models

 - [ChatV1SessionGet200Response](docs/ChatV1SessionGet200Response.md)
 - [ChatV4FriendrequestsGet200Response](docs/ChatV4FriendrequestsGet200Response.md)
 - [ChatV4FriendrequestsGet200ResponseRequestsInner](docs/ChatV4FriendrequestsGet200ResponseRequestsInner.md)
 - [ChatV4FriendrequestsPost200Response](docs/ChatV4FriendrequestsPost200Response.md)
 - [ChatV4FriendsGet200Response](docs/ChatV4FriendsGet200Response.md)
 - [ChatV4FriendsGet200ResponseFriendsInner](docs/ChatV4FriendsGet200ResponseFriendsInner.md)
 - [ChatV4PresencesGet200Response](docs/ChatV4PresencesGet200Response.md)
 - [ChatV4PresencesGet200ResponsePresencesInner](docs/ChatV4PresencesGet200ResponsePresencesInner.md)
 - [ChatV5ParticipantsGet200Response](docs/ChatV5ParticipantsGet200Response.md)
 - [ChatV5ParticipantsGet200ResponseParticipantsInner](docs/ChatV5ParticipantsGet200ResponseParticipantsInner.md)
 - [ChatV6ConversationsAresPartiesGet200Response](docs/ChatV6ConversationsAresPartiesGet200Response.md)
 - [ChatV6ConversationsAresPartiesGet200ResponseConversationsInner](docs/ChatV6ConversationsAresPartiesGet200ResponseConversationsInner.md)
 - [ChatV6ConversationsAresPartiesGet200ResponseConversationsInnerUiState](docs/ChatV6ConversationsAresPartiesGet200ResponseConversationsInnerUiState.md)
 - [ChatV6MessagesGet200Response](docs/ChatV6MessagesGet200Response.md)
 - [ChatV6MessagesGet200ResponseMessagesInner](docs/ChatV6MessagesGet200ResponseMessagesInner.md)
 - [EntitlementsV1TokenGet200Response](docs/EntitlementsV1TokenGet200Response.md)
 - [HelpGet200Response](docs/HelpGet200Response.md)
 - [PlayerAccountAliasesV1ActiveGet200Response](docs/PlayerAccountAliasesV1ActiveGet200Response.md)
 - [ProductSessionV1ExternalSessionsGet200ResponseValue](docs/ProductSessionV1ExternalSessionsGet200ResponseValue.md)
 - [ProductSessionV1ExternalSessionsGet200ResponseValueLaunchConfiguration](docs/ProductSessionV1ExternalSessionsGet200ResponseValueLaunchConfiguration.md)
 - [RiotclientRegionLocaleGet200Response](docs/RiotclientRegionLocaleGet200Response.md)
 - [RsoAuthV1AuthorizationUserinfoGet200Response](docs/RsoAuthV1AuthorizationUserinfoGet200Response.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



