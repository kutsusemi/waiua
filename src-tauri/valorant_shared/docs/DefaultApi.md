# \DefaultApi

All URIs are relative to *https://shared..a.pvp.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_service_v3_content_get**](DefaultApi.md#content_service_v3_content_get) | **GET** /content_service/v3/content | Fetch Content



## content_service_v3_content_get

> models::ContentServiceV3ContentGet200Response content_service_v3_content_get(client_version, client_platform, token, entitlement)
Fetch Content

Get a list of seasons, acts, and events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_version** | **String** |  | [required] |
**client_platform** | **String** |  | [required] |
**token** | **String** |  | [required] |
**entitlement** | **String** |  | [required] |

### Return type

[**models::ContentServiceV3ContentGet200Response**](_content_service_v3_content_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

