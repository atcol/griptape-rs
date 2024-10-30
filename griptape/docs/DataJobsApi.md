# \DataJobsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_data_job**](DataJobsApi.md#create_data_job) | **POST** /data-connectors/{data_connector_id}/data-jobs | 
[**get_data_job**](DataJobsApi.md#get_data_job) | **GET** /data-jobs/{data_job_id} | 
[**list_data_jobs**](DataJobsApi.md#list_data_jobs) | **GET** /data-connectors/{data_connector_id}/data-jobs | 



## create_data_job

> models::CreateDataJobResponseContent create_data_job(data_connector_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_connector_id** | **String** |  | [required] |

### Return type

[**models::CreateDataJobResponseContent**](CreateDataJobResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_job

> models::GetDataJobResponseContent get_data_job(data_job_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_job_id** | **String** |  | [required] |

### Return type

[**models::GetDataJobResponseContent**](GetDataJobResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_data_jobs

> models::ListDataJobsResponseContent list_data_jobs(data_connector_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_connector_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListDataJobsResponseContent**](ListDataJobsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

