# \StructuresApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_structure**](StructuresApi.md#create_structure) | **POST** /structures | 
[**delete_structure**](StructuresApi.md#delete_structure) | **DELETE** /structures/{structure_id} | 
[**get_structure**](StructuresApi.md#get_structure) | **GET** /structures/{structure_id} | 
[**get_structures_dashboard**](StructuresApi.md#get_structures_dashboard) | **GET** /dashboards/structures | 
[**list_structures**](StructuresApi.md#list_structures) | **GET** /structures | 
[**update_structure**](StructuresApi.md#update_structure) | **PATCH** /structures/{structure_id} | 



## create_structure

> models::CreateStructureResponseContent create_structure(create_structure_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_structure_request_content** | [**CreateStructureRequestContent**](CreateStructureRequestContent.md) |  | [required] |

### Return type

[**models::CreateStructureResponseContent**](CreateStructureResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_structure

> delete_structure(structure_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_structure

> models::GetStructureResponseContent get_structure(structure_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_id** | **String** |  | [required] |

### Return type

[**models::GetStructureResponseContent**](GetStructureResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_structures_dashboard

> models::GetStructuresDashboardResponseContent get_structures_dashboard(start_time, end_time, period, structure_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | Option<**String**> |  |  |
**end_time** | Option<**String**> |  |  |
**period** | Option<[**Period**](.md)> |  |  |
**structure_ids** | Option<**String**> |  |  |

### Return type

[**models::GetStructuresDashboardResponseContent**](GetStructuresDashboardResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_structures

> models::ListStructuresResponseContent list_structures(page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListStructuresResponseContent**](ListStructuresResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_structure

> models::UpdateStructureResponseContent update_structure(structure_id, update_structure_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_id** | **String** |  | [required] |
**update_structure_request_content** | Option<[**UpdateStructureRequestContent**](UpdateStructureRequestContent.md)> |  |  |

### Return type

[**models::UpdateStructureResponseContent**](UpdateStructureResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

