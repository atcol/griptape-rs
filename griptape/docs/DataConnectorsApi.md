# \DataConnectorsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_data_connector**](DataConnectorsApi.md#create_data_connector) | **POST** /data-connectors | 
[**delete_data_connector**](DataConnectorsApi.md#delete_data_connector) | **DELETE** /data-connectors/{data_connector_id} | 
[**get_data_connector**](DataConnectorsApi.md#get_data_connector) | **GET** /data-connectors/{data_connector_id} | 
[**list_data_connectors**](DataConnectorsApi.md#list_data_connectors) | **GET** /data-connectors | 
[**update_data_connector**](DataConnectorsApi.md#update_data_connector) | **PATCH** /data-connectors/{data_connector_id} | 



## create_data_connector

> models::CreateDataConnectorResponseContent create_data_connector(create_data_connector_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_data_connector_request_content** | [**CreateDataConnectorRequestContent**](CreateDataConnectorRequestContent.md) |  | [required] |

### Return type

[**models::CreateDataConnectorResponseContent**](CreateDataConnectorResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_connector

> delete_data_connector(data_connector_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_connector_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_connector

> models::GetDataConnectorResponseContent get_data_connector(data_connector_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_connector_id** | **String** |  | [required] |

### Return type

[**models::GetDataConnectorResponseContent**](GetDataConnectorResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_data_connectors

> models::ListDataConnectorsResponseContent list_data_connectors(page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListDataConnectorsResponseContent**](ListDataConnectorsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_data_connector

> models::UpdateDataConnectorResponseContent update_data_connector(data_connector_id, update_data_connector_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_connector_id** | **String** |  | [required] |
**update_data_connector_request_content** | Option<[**UpdateDataConnectorRequestContent**](UpdateDataConnectorRequestContent.md)> |  |  |

### Return type

[**models::UpdateDataConnectorResponseContent**](UpdateDataConnectorResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

