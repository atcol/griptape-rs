# \StructureRunsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_structure_run**](StructureRunsApi.md#cancel_structure_run) | **POST** /structure-runs/{structure_run_id}/cancel | 
[**create_structure_run**](StructureRunsApi.md#create_structure_run) | **POST** /structures/{structure_id}/runs | 
[**get_structure_run**](StructureRunsApi.md#get_structure_run) | **GET** /structure-runs/{structure_run_id} | 
[**list_structure_run_logs**](StructureRunsApi.md#list_structure_run_logs) | **GET** /structure-runs/{structure_run_id}/logs | 
[**list_structure_runs**](StructureRunsApi.md#list_structure_runs) | **GET** /structures/{structure_id}/runs | 



## cancel_structure_run

> models::CancelStructureRunResponseContent cancel_structure_run(structure_run_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_run_id** | **String** |  | [required] |

### Return type

[**models::CancelStructureRunResponseContent**](CancelStructureRunResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_structure_run

> models::CreateStructureRunResponseContent create_structure_run(structure_id, create_structure_run_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_id** | **String** |  | [required] |
**create_structure_run_request_content** | [**CreateStructureRunRequestContent**](CreateStructureRunRequestContent.md) |  | [required] |

### Return type

[**models::CreateStructureRunResponseContent**](CreateStructureRunResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_structure_run

> models::GetStructureRunResponseContent get_structure_run(structure_run_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_run_id** | **String** |  | [required] |

### Return type

[**models::GetStructureRunResponseContent**](GetStructureRunResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_structure_run_logs

> models::ListStructureRunLogsResponseContent list_structure_run_logs(structure_run_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_run_id** | **String** |  | [required] |

### Return type

[**models::ListStructureRunLogsResponseContent**](ListStructureRunLogsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_structure_runs

> models::ListStructureRunsResponseContent list_structure_runs(structure_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListStructureRunsResponseContent**](ListStructureRunsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

