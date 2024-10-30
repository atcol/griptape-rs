# \AssistantsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_assistant**](AssistantsApi.md#create_assistant) | **POST** /assistants | 
[**delete_assistant**](AssistantsApi.md#delete_assistant) | **DELETE** /assistants/{assistant_id} | 
[**get_assistant**](AssistantsApi.md#get_assistant) | **GET** /assistants/{assistant_id} | 
[**list_assistants**](AssistantsApi.md#list_assistants) | **GET** /assistants | 
[**update_assistant**](AssistantsApi.md#update_assistant) | **PATCH** /assistants/{assistant_id} | 



## create_assistant

> models::CreateAssistantResponseContent create_assistant(create_assistant_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_assistant_request_content** | [**CreateAssistantRequestContent**](CreateAssistantRequestContent.md) |  | [required] |

### Return type

[**models::CreateAssistantResponseContent**](CreateAssistantResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_assistant

> delete_assistant(assistant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assistant

> models::GetAssistantResponseContent get_assistant(assistant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** |  | [required] |

### Return type

[**models::GetAssistantResponseContent**](GetAssistantResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_assistants

> models::ListAssistantsResponseContent list_assistants(page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListAssistantsResponseContent**](ListAssistantsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_assistant

> models::UpdateAssistantResponseContent update_assistant(assistant_id, update_assistant_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** |  | [required] |
**update_assistant_request_content** | Option<[**UpdateAssistantRequestContent**](UpdateAssistantRequestContent.md)> |  |  |

### Return type

[**models::UpdateAssistantResponseContent**](UpdateAssistantResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

