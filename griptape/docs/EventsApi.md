# \EventsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_events**](EventsApi.md#create_events) | **POST** /structure-runs/{structure_run_id}/events | 
[**get_event**](EventsApi.md#get_event) | **GET** /events/{event_id} | 
[**list_assistant_events**](EventsApi.md#list_assistant_events) | **GET** /assistant-runs/{assistant_run_id}/events | 
[**list_events**](EventsApi.md#list_events) | **GET** /structure-runs/{structure_run_id}/events | 



## create_events

> create_events(structure_run_id, create_events_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_run_id** | **String** |  | [required] |
**create_events_request_content** | Option<[**CreateEventsRequestContent**](CreateEventsRequestContent.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event

> models::GetEventResponseContent get_event(event_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** |  | [required] |

### Return type

[**models::GetEventResponseContent**](GetEventResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_assistant_events

> models::ListAssistantEventsResponseContent list_assistant_events(assistant_run_id, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_run_id** | **String** |  | [required] |
**limit** | Option<**String**> |  |  |
**offset** | Option<**String**> |  |  |

### Return type

[**models::ListAssistantEventsResponseContent**](ListAssistantEventsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_events

> models::ListEventsResponseContent list_events(structure_run_id, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_run_id** | **String** |  | [required] |
**limit** | Option<**String**> |  |  |
**offset** | Option<**String**> |  |  |

### Return type

[**models::ListEventsResponseContent**](ListEventsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

