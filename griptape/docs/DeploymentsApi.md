# \DeploymentsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deployment**](DeploymentsApi.md#create_deployment) | **POST** /structures/{structure_id}/deployments | 
[**get_deployment**](DeploymentsApi.md#get_deployment) | **GET** /deployments/{deployment_id} | 
[**list_deployments**](DeploymentsApi.md#list_deployments) | **GET** /structures/{structure_id}/deployments | 



## create_deployment

> models::CreateDeploymentResponseContent create_deployment(structure_id, create_deployment_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_id** | **String** |  | [required] |
**create_deployment_request_content** | Option<[**CreateDeploymentRequestContent**](CreateDeploymentRequestContent.md)> |  |  |

### Return type

[**models::CreateDeploymentResponseContent**](CreateDeploymentResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment

> models::GetDeploymentResponseContent get_deployment(deployment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** |  | [required] |

### Return type

[**models::GetDeploymentResponseContent**](GetDeploymentResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deployments

> models::ListDeploymentsResponseContent list_deployments(structure_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListDeploymentsResponseContent**](ListDeploymentsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

