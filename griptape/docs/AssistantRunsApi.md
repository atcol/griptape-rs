# \AssistantRunsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_assistant_run**](AssistantRunsApi.md#create_assistant_run) | **POST** /assistants/{assistant_id}/runs | 
[**get_assistant_run**](AssistantRunsApi.md#get_assistant_run) | **GET** /assistant-runs/{assistant_run_id} | 
[**list_assistant_runs**](AssistantRunsApi.md#list_assistant_runs) | **GET** /assistants/{assistant_id}/runs | 



## create_assistant_run

> models::CreateAssistantRunResponseContent create_assistant_run(assistant_id, create_assistant_run_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** |  | [required] |
**create_assistant_run_request_content** | Option<[**CreateAssistantRunRequestContent**](CreateAssistantRunRequestContent.md)> |  |  |

### Return type

[**models::CreateAssistantRunResponseContent**](CreateAssistantRunResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assistant_run

> models::GetAssistantRunResponseContent get_assistant_run(assistant_run_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_run_id** | **String** |  | [required] |

### Return type

[**models::GetAssistantRunResponseContent**](GetAssistantRunResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_assistant_runs

> models::ListAssistantRunsResponseContent list_assistant_runs(assistant_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListAssistantRunsResponseContent**](ListAssistantRunsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

