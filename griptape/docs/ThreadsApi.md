# \ThreadsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_thread**](ThreadsApi.md#create_thread) | **POST** /threads | 
[**delete_thread**](ThreadsApi.md#delete_thread) | **DELETE** /threads/{thread_id} | 
[**get_thread**](ThreadsApi.md#get_thread) | **GET** /threads/{thread_id} | 
[**list_threads**](ThreadsApi.md#list_threads) | **GET** /threads | 
[**update_thread**](ThreadsApi.md#update_thread) | **PATCH** /threads/{thread_id} | 



## create_thread

> models::CreateThreadResponseContent create_thread(create_thread_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_thread_request_content** | [**CreateThreadRequestContent**](CreateThreadRequestContent.md) |  | [required] |

### Return type

[**models::CreateThreadResponseContent**](CreateThreadResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_thread

> delete_thread(thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thread

> models::GetThreadResponseContent get_thread(thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |

### Return type

[**models::GetThreadResponseContent**](GetThreadResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_threads

> models::ListThreadsResponseContent list_threads(page, page_size, alias)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |
**alias** | Option<**String**> |  |  |

### Return type

[**models::ListThreadsResponseContent**](ListThreadsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_thread

> models::UpdateThreadResponseContent update_thread(thread_id, update_thread_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |
**update_thread_request_content** | Option<[**UpdateThreadRequestContent**](UpdateThreadRequestContent.md)> |  |  |

### Return type

[**models::UpdateThreadResponseContent**](UpdateThreadResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

