# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_riotgames_com_api_v1_authorization_post**](DefaultApi.md#auth_riotgames_com_api_v1_authorization_post) | **POST** //auth.riotgames.com/api/v1/authorization | Auth Cookies
[**auth_riotgames_com_api_v1_authorization_put**](DefaultApi.md#auth_riotgames_com_api_v1_authorization_put) | **PUT** //auth.riotgames.com/api/v1/authorization | Multi-Factor Authentication
[**auth_riotgames_com_authorizeredirect_urihttps3_a2_f2_fplayvalorant_com2_fopt_inclient_idplay_valorant_web_prodresponse_typetoken20id_tokennonce1scopeaccount20openid_get**](DefaultApi.md#auth_riotgames_com_authorizeredirect_urihttps3_a2_f2_fplayvalorant_com2_fopt_inclient_idplay_valorant_web_prodresponse_typetoken20id_tokennonce1scopeaccount20openid_get) | **GET** //auth.riotgames.com/authorize?redirect_uri=https%3A%2F%2Fplayvalorant.com%2Fopt_in&client_id=play-valorant-web-prod&response_type=token%20id_token&nonce=1&scope=account%20openid | Cookie Reauth
[**auth_riotgames_com_userinfo_get**](DefaultApi.md#auth_riotgames_com_userinfo_get) | **GET** //auth.riotgames.com/userinfo | Player Info
[**clientconfig_rpg_riotgames_com_api_v1_config_playerapp_riot20_client_get**](DefaultApi.md#clientconfig_rpg_riotgames_com_api_v1_config_playerapp_riot20_client_get) | **GET** //clientconfig.rpg.riotgames.com/api/v1/config/player?app=Riot%20Client | Riot Client Config
[**entitlements_auth_riotgames_com_api_token_v1_post**](DefaultApi.md#entitlements_auth_riotgames_com_api_token_v1_post) | **POST** //entitlements.auth.riotgames.com/api/token/v1 | Entitlement
[**riot_geo_pas_si_riotgames_com_pas_v1_product_valorant_put**](DefaultApi.md#riot_geo_pas_si_riotgames_com_pas_v1_product_valorant_put) | **PUT** //riot-geo.pas.si.riotgames.com/pas/v1/product/valorant | Riot Geo
[**riot_geo_pas_si_riotgames_com_pas_v1_service_chat_get**](DefaultApi.md#riot_geo_pas_si_riotgames_com_pas_v1_service_chat_get) | **GET** //riot-geo.pas.si.riotgames.com/pas/v1/service/chat | PAS Token



## auth_riotgames_com_api_v1_authorization_post

> models::AuthRiotgamesComApiV1AuthorizationPut200Response auth_riotgames_com_api_v1_authorization_post()
Auth Cookies

Prepare cookies for auth request

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthRiotgamesComApiV1AuthorizationPut200Response**](__auth_riotgames_com_api_v1_authorization_put_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_riotgames_com_api_v1_authorization_put

> models::AuthRiotgamesComApiV1AuthorizationPut200Response auth_riotgames_com_api_v1_authorization_put()
Multi-Factor Authentication

Submits a multi-factor authentication code for login

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthRiotgamesComApiV1AuthorizationPut200Response**](__auth_riotgames_com_api_v1_authorization_put_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_riotgames_com_authorizeredirect_urihttps3_a2_f2_fplayvalorant_com2_fopt_inclient_idplay_valorant_web_prodresponse_typetoken20id_tokennonce1scopeaccount20openid_get

> models::AuthRiotgamesComApiV1AuthorizationPut200Response auth_riotgames_com_authorizeredirect_urihttps3_a2_f2_fplayvalorant_com2_fopt_inclient_idplay_valorant_web_prodresponse_typetoken20id_tokennonce1scopeaccount20openid_get()
Cookie Reauth

Get a new token using the cookies from a previous authorization request   Use the saved cookies from [PUT Auth Request] (specifically the `ssid` cookie). The auth token and id token can be found from the url this request redirects to.      It's recommended to use this endpoint instead of storing the password and sending it again.      There are ongoing tests at documented at <https://github.com/techchrism/riot-auth-test> that test for auth lifespan using different cookie strategies.   Currently, it appears refreshing with just the `ssid` cookie is only stable for one week and refreshing with all auth cookies is stable for three weeks.      On a successful response, the 301 redirect location header will be of the format: > ```https://playvalorant.com/opt_in#access_token={access token}&scope=openid&iss=https%3A%2F%2Fauth.riotgames.com&id_token={id token}&token_type=Bearer&session_state={session state}&expires_in=3600```    On an unsuccessful response, the 301 redirect location header will be of the format: > ```https://authenticate.riotgames.com/login?client_id=play-valorant-web-prod&nonce=1&redirect_uri=https%3A%2F%2Fauth.riotgames.com%2Fauthorize%3Fclient_id%3Dplay-valorant-web-prod%26nonce%3D1%26redirect_uri%3Dhttps%253A%252F%252Fplayvalorant.com%252Fopt_in%26response_type%3Dtoken%2520id_token&response_type=token%20id_token&method=riot_identity``` 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthRiotgamesComApiV1AuthorizationPut200Response**](__auth_riotgames_com_api_v1_authorization_put_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_riotgames_com_userinfo_get

> models::AuthRiotgamesComUserinfoGet200Response auth_riotgames_com_userinfo_get()
Player Info

Get the PUUID and other info from a token

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthRiotgamesComUserinfoGet200Response**](__auth_riotgames_com_userinfo_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clientconfig_rpg_riotgames_com_api_v1_config_playerapp_riot20_client_get

> models::ClientconfigRpgRiotgamesComApiV1ConfigPlayerAppRiot20ClientGet200Response clientconfig_rpg_riotgames_com_api_v1_config_playerapp_riot20_client_get(x_riot_entitlements_jwt)
Riot Client Config

Gets the config file used by the Riot Client. This includes a ton of info, most of it undocumented.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_riot_entitlements_jwt** | **String** |  | [required] |

### Return type

[**models::ClientconfigRpgRiotgamesComApiV1ConfigPlayerAppRiot20ClientGet200Response**](__clientconfig_rpg_riotgames_com_api_v1_config_player_app_Riot_20Client_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## entitlements_auth_riotgames_com_api_token_v1_post

> models::EntitlementsAuthRiotgamesComApiTokenV1Post200Response entitlements_auth_riotgames_com_api_token_v1_post()
Entitlement

Get entitlement for remote requests with a token

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::EntitlementsAuthRiotgamesComApiTokenV1Post200Response**](__entitlements_auth_riotgames_com_api_token_v1_post_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## riot_geo_pas_si_riotgames_com_pas_v1_product_valorant_put

> models::RiotGeoPasSiRiotgamesComPasV1ProductValorantPut200Response riot_geo_pas_si_riotgames_com_pas_v1_product_valorant_put()
Riot Geo

Get the region for a given ID token and auth token. The ID token and auth token can be obtained from [PUT Cookie Reauth]

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RiotGeoPasSiRiotgamesComPasV1ProductValorantPut200Response**](__riot_geo_pas_si_riotgames_com_pas_v1_product_valorant_put_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## riot_geo_pas_si_riotgames_com_pas_v1_service_chat_get

> String riot_geo_pas_si_riotgames_com_pas_v1_service_chat_get()
PAS Token

Get a PAS token using the auth token. The PAS token is a JWT that contains the affinity for the XMPP server.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

