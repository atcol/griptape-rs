# \AssetsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_asset**](AssetsApi.md#create_asset) | **PUT** /buckets/{bucket_id}/assets | 
[**create_asset_url**](AssetsApi.md#create_asset_url) | **POST** /buckets/{bucket_id}/asset-urls/{name} | 
[**delete_asset**](AssetsApi.md#delete_asset) | **DELETE** /buckets/{bucket_id}/assets/{name} | 
[**get_asset**](AssetsApi.md#get_asset) | **GET** /buckets/{bucket_id}/assets/{name} | 
[**list_assets**](AssetsApi.md#list_assets) | **GET** /buckets/{bucket_id}/assets | 



## create_asset

> models::CreateAssetResponseContent create_asset(bucket_id, create_asset_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** |  | [required] |
**create_asset_request_content** | [**CreateAssetRequestContent**](CreateAssetRequestContent.md) |  | [required] |

### Return type

[**models::CreateAssetResponseContent**](CreateAssetResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_asset_url

> models::CreateAssetUrlResponseContent create_asset_url(name, bucket_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bucket_id** | **String** |  | [required] |

### Return type

[**models::CreateAssetUrlResponseContent**](CreateAssetUrlResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_asset

> delete_asset(name, bucket_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bucket_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset

> models::GetAssetResponseContent get_asset(name, bucket_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bucket_id** | **String** |  | [required] |

### Return type

[**models::GetAssetResponseContent**](GetAssetResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_assets

> models::ListAssetsResponseContent list_assets(bucket_id, page, page_size, prefix, postfix)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |
**prefix** | Option<**String**> |  |  |
**postfix** | Option<**String**> |  |  |

### Return type

[**models::ListAssetsResponseContent**](ListAssetsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

