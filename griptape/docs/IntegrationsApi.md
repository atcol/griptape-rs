# \IntegrationsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_integration**](IntegrationsApi.md#create_integration) | **POST** /integrations | 
[**delete_integration**](IntegrationsApi.md#delete_integration) | **DELETE** /integrations/{integration_id} | 
[**get_integration**](IntegrationsApi.md#get_integration) | **GET** /integrations/{integration_id} | 
[**list_integrations**](IntegrationsApi.md#list_integrations) | **GET** /integrations | 
[**update_integration**](IntegrationsApi.md#update_integration) | **PATCH** /integrations/{integration_id} | 



## create_integration

> models::CreateIntegrationResponseContent create_integration(create_integration_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_integration_request_content** | [**CreateIntegrationRequestContent**](CreateIntegrationRequestContent.md) |  | [required] |

### Return type

[**models::CreateIntegrationResponseContent**](CreateIntegrationResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_integration

> delete_integration(integration_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integration

> models::GetIntegrationResponseContent get_integration(integration_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** |  | [required] |

### Return type

[**models::GetIntegrationResponseContent**](GetIntegrationResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_integrations

> models::ListIntegrationsResponseContent list_integrations(page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListIntegrationsResponseContent**](ListIntegrationsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_integration

> models::UpdateIntegrationResponseContent update_integration(integration_id, update_integration_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** |  | [required] |
**update_integration_request_content** | Option<[**UpdateIntegrationRequestContent**](UpdateIntegrationRequestContent.md)> |  |  |

### Return type

[**models::UpdateIntegrationResponseContent**](UpdateIntegrationResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

