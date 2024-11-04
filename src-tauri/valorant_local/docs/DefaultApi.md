# \DefaultApi

All URIs are relative to *https://127.0.0.1:*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_v1_session_get**](DefaultApi.md#chat_v1_session_get) | **GET** /chat/v1/session | Chat Session
[**chat_v4_friendrequests_delete**](DefaultApi.md#chat_v4_friendrequests_delete) | **DELETE** /chat/v4/friendrequests | Remove Friend Request
[**chat_v4_friendrequests_get**](DefaultApi.md#chat_v4_friendrequests_get) | **GET** /chat/v4/friendrequests | Friend Requests
[**chat_v4_friendrequests_post**](DefaultApi.md#chat_v4_friendrequests_post) | **POST** /chat/v4/friendrequests | Send Friend Request
[**chat_v4_friends_get**](DefaultApi.md#chat_v4_friends_get) | **GET** /chat/v4/friends | Friends
[**chat_v4_presences_get**](DefaultApi.md#chat_v4_presences_get) | **GET** /chat/v4/presences | Presence
[**chat_v5_participants_get**](DefaultApi.md#chat_v5_participants_get) | **GET** /chat/v5/participants | Chat Participants
[**chat_v6_conversations_ares_coregame_get**](DefaultApi.md#chat_v6_conversations_ares_coregame_get) | **GET** /chat/v6/conversations/ares_coregame | Current Game Chat Info
[**chat_v6_conversations_ares_parties_get**](DefaultApi.md#chat_v6_conversations_ares_parties_get) | **GET** /chat/v6/conversations/ares_parties | Party Chat Info
[**chat_v6_conversations_ares_pregame_get**](DefaultApi.md#chat_v6_conversations_ares_pregame_get) | **GET** /chat/v6/conversations/ares_pregame | Pre-Game Chat Info
[**chat_v6_conversations_get**](DefaultApi.md#chat_v6_conversations_get) | **GET** /chat/v6/conversations | All Chat Info
[**chat_v6_messages_get**](DefaultApi.md#chat_v6_messages_get) | **GET** /chat/v6/messages | Chat History
[**chat_v6_messages_post**](DefaultApi.md#chat_v6_messages_post) | **POST** /chat/v6/messages | Send Chat
[**entitlements_v1_token_get**](DefaultApi.md#entitlements_v1_token_get) | **GET** /entitlements/v1/token | Entitlements Token
[**help_get**](DefaultApi.md#help_get) | **GET** /help | Local Help
[**player_account_aliases_v1_active_get**](DefaultApi.md#player_account_aliases_v1_active_get) | **GET** /player_account/aliases/v1/active | Account Alias
[**product_session_v1_external_sessions_get**](DefaultApi.md#product_session_v1_external_sessions_get) | **GET** /product_session/v1/external_sessions | Sessions
[**riotclient_region_locale_get**](DefaultApi.md#riotclient_region_locale_get) | **GET** /riotclient/region_locale | Client Region
[**rso_auth_v1_authorization_userinfo_get**](DefaultApi.md#rso_auth_v1_authorization_userinfo_get) | **GET** /rso_auth/v1/authorization/userinfo | RSO User Info
[**swagger_v3_openapi_json_get**](DefaultApi.md#swagger_v3_openapi_json_get) | **GET** /swagger/v3/openapi.json | Local Swagger Docs



## chat_v1_session_get

> models::ChatV1SessionGet200Response chat_v1_session_get()
Chat Session

Get the current session including player name and PUUID

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV1SessionGet200Response**](_chat_v1_session_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v4_friendrequests_delete

> serde_json::Value chat_v4_friendrequests_delete()
Remove Friend Request

Removes an outgoing friend request

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v4_friendrequests_get

> models::ChatV4FriendrequestsGet200Response chat_v4_friendrequests_get()
Friend Requests

Get a list of friend requests

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV4FriendrequestsGet200Response**](_chat_v4_friendrequests_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v4_friendrequests_post

> models::ChatV4FriendrequestsPost200Response chat_v4_friendrequests_post()
Send Friend Request

Sends a friend request to a player. Can be used in conjunction with [GET Friend Requests] and [DELETE Remove Friend Request] to determine a player's PUUID from their game name.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV4FriendrequestsPost200Response**](_chat_v4_friendrequests_post_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v4_friends_get

> models::ChatV4FriendsGet200Response chat_v4_friends_get()
Friends

Get a list of friends

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV4FriendsGet200Response**](_chat_v4_friends_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v4_presences_get

> models::ChatV4PresencesGet200Response chat_v4_presences_get()
Presence

Get a list of online friends and their activity   If the player is playing Valorant, `private` is a base64-encoded JSON string that contains useful information such as party and in-progress game score.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV4PresencesGet200Response**](_chat_v4_presences_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v5_participants_get

> models::ChatV5ParticipantsGet200Response chat_v5_participants_get()
Chat Participants

Get information about the participants of all active conversations or a specific conversation if a cid is provided

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV5ParticipantsGet200Response**](_chat_v5_participants_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v6_conversations_ares_coregame_get

> models::ChatV6ConversationsAresPartiesGet200Response chat_v6_conversations_ares_coregame_get()
Current Game Chat Info

Get information about the current game chat

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV6ConversationsAresPartiesGet200Response**](_chat_v6_conversations_ares_parties_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v6_conversations_ares_parties_get

> models::ChatV6ConversationsAresPartiesGet200Response chat_v6_conversations_ares_parties_get()
Party Chat Info

Get information about the party chat

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV6ConversationsAresPartiesGet200Response**](_chat_v6_conversations_ares_parties_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v6_conversations_ares_pregame_get

> models::ChatV6ConversationsAresPartiesGet200Response chat_v6_conversations_ares_pregame_get()
Pre-Game Chat Info

Get information about the pre-game chat

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV6ConversationsAresPartiesGet200Response**](_chat_v6_conversations_ares_parties_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v6_conversations_get

> models::ChatV6ConversationsAresPartiesGet200Response chat_v6_conversations_get()
All Chat Info

Get information about all active conversations

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV6ConversationsAresPartiesGet200Response**](_chat_v6_conversations_ares_parties_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v6_messages_get

> models::ChatV6MessagesGet200Response chat_v6_messages_get()
Chat History

Get chat history for all conversations or a specific conversation if the cid is provided

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV6MessagesGet200Response**](_chat_v6_messages_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_v6_messages_post

> models::ChatV6MessagesGet200Response chat_v6_messages_post()
Send Chat

Send a message to the specified group

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChatV6MessagesGet200Response**](_chat_v6_messages_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## entitlements_v1_token_get

> models::EntitlementsV1TokenGet200Response entitlements_v1_token_get()
Entitlements Token

Gets both the token and entitlement for API usage `accessToken` is used as the token and `token` is used as the entitlement.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::EntitlementsV1TokenGet200Response**](_entitlements_v1_token_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## help_get

> models::HelpGet200Response help_get()
Local Help

Get help for the local client

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HelpGet200Response**](_help_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_account_aliases_v1_active_get

> models::PlayerAccountAliasesV1ActiveGet200Response player_account_aliases_v1_active_get()
Account Alias

Gets the player username and tagline

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PlayerAccountAliasesV1ActiveGet200Response**](_player_account_aliases_v1_active_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## product_session_v1_external_sessions_get

> std::collections::HashMap<String, models::ProductSessionV1ExternalSessionsGet200ResponseValue> product_session_v1_external_sessions_get()
Sessions

Gets info about the running Valorant process including start arguments   Can be used to get shard, region, and puuid by parsing launch args.

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, models::ProductSessionV1ExternalSessionsGet200ResponseValue>**](_product_session_v1_external_sessions_get_200_response_value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## riotclient_region_locale_get

> models::RiotclientRegionLocaleGet200Response riotclient_region_locale_get()
Client Region

Gets info about the region and locale from the Riot client

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RiotclientRegionLocaleGet200Response**](_riotclient_region_locale_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rso_auth_v1_authorization_userinfo_get

> models::RsoAuthV1AuthorizationUserinfoGet200Response rso_auth_v1_authorization_userinfo_get()
RSO User Info

Get RSO user info

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RsoAuthV1AuthorizationUserinfoGet200Response**](_rso_auth_v1_authorization_userinfo_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swagger_v3_openapi_json_get

> serde_json::Value swagger_v3_openapi_json_get()
Local Swagger Docs

Fetches json Swagger docs for local endpoints. Can be imported into Swagger or Insomnia.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

