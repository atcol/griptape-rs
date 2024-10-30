# \ApiKeysApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](ApiKeysApi.md#create_api_key) | **POST** /users/{user_id}/api-keys | 
[**delete_api_key**](ApiKeysApi.md#delete_api_key) | **DELETE** /api-keys/{api_key_id} | 
[**get_api_key**](ApiKeysApi.md#get_api_key) | **GET** /api-keys/{api_key_id} | 
[**list_api_keys**](ApiKeysApi.md#list_api_keys) | **GET** /users/{user_id}/api-keys | 
[**update_api_key**](ApiKeysApi.md#update_api_key) | **PATCH** /api-keys/{api_key_id} | 



## create_api_key

> models::CreateApiKeyResponseContent create_api_key(user_id, create_api_key_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**create_api_key_request_content** | [**CreateApiKeyRequestContent**](CreateApiKeyRequestContent.md) |  | [required] |

### Return type

[**models::CreateApiKeyResponseContent**](CreateApiKeyResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(api_key_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key

> models::GetApiKeyResponseContent get_api_key(api_key_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** |  | [required] |

### Return type

[**models::GetApiKeyResponseContent**](GetApiKeyResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_api_keys

> models::ListApiKeysResponseContent list_api_keys(user_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListApiKeysResponseContent**](ListApiKeysResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key

> models::UpdateApiKeyResponseContent update_api_key(api_key_id, update_api_key_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** |  | [required] |
**update_api_key_request_content** | Option<[**UpdateApiKeyRequestContent**](UpdateApiKeyRequestContent.md)> |  |  |

### Return type

[**models::UpdateApiKeyResponseContent**](UpdateApiKeyResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

