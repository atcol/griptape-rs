# \MessagesApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_message**](MessagesApi.md#create_message) | **POST** /threads/{thread_id}/messages | 
[**delete_message**](MessagesApi.md#delete_message) | **DELETE** /messages/{message_id} | 
[**get_message**](MessagesApi.md#get_message) | **GET** /messages/{message_id} | 
[**list_messages**](MessagesApi.md#list_messages) | **GET** /threads/{thread_id}/messages | 
[**update_message**](MessagesApi.md#update_message) | **PATCH** /messages/{message_id} | 



## create_message

> models::CreateMessageResponseContent create_message(thread_id, create_message_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |
**create_message_request_content** | [**CreateMessageRequestContent**](CreateMessageRequestContent.md) |  | [required] |

### Return type

[**models::CreateMessageResponseContent**](CreateMessageResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message

> delete_message(message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message

> models::GetMessageResponseContent get_message(message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** |  | [required] |

### Return type

[**models::GetMessageResponseContent**](GetMessageResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_messages

> models::ListMessagesResponseContent list_messages(thread_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListMessagesResponseContent**](ListMessagesResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_message

> models::UpdateMessageResponseContent update_message(message_id, update_message_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** |  | [required] |
**update_message_request_content** | Option<[**UpdateMessageRequestContent**](UpdateMessageRequestContent.md)> |  |  |

### Return type

[**models::UpdateMessageResponseContent**](UpdateMessageResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

