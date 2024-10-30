# \BucketsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bucket**](BucketsApi.md#create_bucket) | **POST** /buckets | 
[**delete_bucket**](BucketsApi.md#delete_bucket) | **DELETE** /buckets/{bucket_id} | 
[**get_bucket**](BucketsApi.md#get_bucket) | **GET** /buckets/{bucket_id} | 
[**list_buckets**](BucketsApi.md#list_buckets) | **GET** /buckets | 
[**update_bucket**](BucketsApi.md#update_bucket) | **PATCH** /buckets/{bucket_id} | 



## create_bucket

> models::CreateBucketResponseContent create_bucket(create_bucket_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_bucket_request_content** | [**CreateBucketRequestContent**](CreateBucketRequestContent.md) |  | [required] |

### Return type

[**models::CreateBucketResponseContent**](CreateBucketResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bucket

> delete_bucket(bucket_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bucket

> models::GetBucketResponseContent get_bucket(bucket_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** |  | [required] |

### Return type

[**models::GetBucketResponseContent**](GetBucketResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_buckets

> models::ListBucketsResponseContent list_buckets(page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListBucketsResponseContent**](ListBucketsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_bucket

> models::UpdateBucketResponseContent update_bucket(bucket_id, update_bucket_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** |  | [required] |
**update_bucket_request_content** | Option<[**UpdateBucketRequestContent**](UpdateBucketRequestContent.md)> |  |  |

### Return type

[**models::UpdateBucketResponseContent**](UpdateBucketResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

