# \KnowledgeBasesApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_knowledge_base**](KnowledgeBasesApi.md#create_knowledge_base) | **POST** /knowledge-bases | 
[**delete_knowledge_base**](KnowledgeBasesApi.md#delete_knowledge_base) | **DELETE** /knowledge-bases/{knowledge_base_id} | 
[**get_knowledge_base**](KnowledgeBasesApi.md#get_knowledge_base) | **GET** /knowledge-bases/{knowledge_base_id} | 
[**get_knowledge_base_query**](KnowledgeBasesApi.md#get_knowledge_base_query) | **GET** /knowledge-base-queries/{knowledge_base_query_id} | 
[**get_knowledge_base_search**](KnowledgeBasesApi.md#get_knowledge_base_search) | **GET** /knowledge-base-searches/{knowledge_base_search_id} | 
[**list_knowledge_base_queries**](KnowledgeBasesApi.md#list_knowledge_base_queries) | **GET** /knowledge-bases/{knowledge_base_id}/queries | 
[**list_knowledge_base_searches**](KnowledgeBasesApi.md#list_knowledge_base_searches) | **GET** /knowledge-bases/{knowledge_base_id}/searches | 
[**list_knowledge_bases**](KnowledgeBasesApi.md#list_knowledge_bases) | **GET** /knowledge-bases | 
[**query_knowledge_base**](KnowledgeBasesApi.md#query_knowledge_base) | **POST** /knowledge-bases/{knowledge_base_id}/query | 
[**search_knowledge_base**](KnowledgeBasesApi.md#search_knowledge_base) | **POST** /knowledge-bases/{knowledge_base_id}/search | 
[**update_knowledge_base**](KnowledgeBasesApi.md#update_knowledge_base) | **PATCH** /knowledge-bases/{knowledge_base_id} | 



## create_knowledge_base

> models::CreateKnowledgeBaseResponseContent create_knowledge_base(create_knowledge_base_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_knowledge_base_request_content** | [**CreateKnowledgeBaseRequestContent**](CreateKnowledgeBaseRequestContent.md) |  | [required] |

### Return type

[**models::CreateKnowledgeBaseResponseContent**](CreateKnowledgeBaseResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_knowledge_base

> delete_knowledge_base(knowledge_base_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_base

> models::GetKnowledgeBaseResponseContent get_knowledge_base(knowledge_base_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** |  | [required] |

### Return type

[**models::GetKnowledgeBaseResponseContent**](GetKnowledgeBaseResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_base_query

> models::GetKnowledgeBaseQueryResponseContent get_knowledge_base_query(knowledge_base_query_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_query_id** | **String** |  | [required] |

### Return type

[**models::GetKnowledgeBaseQueryResponseContent**](GetKnowledgeBaseQueryResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_base_search

> models::GetKnowledgeBaseSearchResponseContent get_knowledge_base_search(knowledge_base_search_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_search_id** | **String** |  | [required] |

### Return type

[**models::GetKnowledgeBaseSearchResponseContent**](GetKnowledgeBaseSearchResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_knowledge_base_queries

> models::ListKnowledgeBaseQueriesResponseContent list_knowledge_base_queries(knowledge_base_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListKnowledgeBaseQueriesResponseContent**](ListKnowledgeBaseQueriesResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_knowledge_base_searches

> models::ListKnowledgeBaseSearchesResponseContent list_knowledge_base_searches(knowledge_base_id, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListKnowledgeBaseSearchesResponseContent**](ListKnowledgeBaseSearchesResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_knowledge_bases

> models::ListKnowledgeBasesResponseContent list_knowledge_bases(page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |

### Return type

[**models::ListKnowledgeBasesResponseContent**](ListKnowledgeBasesResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_knowledge_base

> models::QueryKnowledgeBaseResponseContent query_knowledge_base(knowledge_base_id, query_knowledge_base_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** |  | [required] |
**query_knowledge_base_request_content** | [**QueryKnowledgeBaseRequestContent**](QueryKnowledgeBaseRequestContent.md) |  | [required] |

### Return type

[**models::QueryKnowledgeBaseResponseContent**](QueryKnowledgeBaseResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_knowledge_base

> models::SearchKnowledgeBaseResponseContent search_knowledge_base(knowledge_base_id, search_knowledge_base_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** |  | [required] |
**search_knowledge_base_request_content** | [**SearchKnowledgeBaseRequestContent**](SearchKnowledgeBaseRequestContent.md) |  | [required] |

### Return type

[**models::SearchKnowledgeBaseResponseContent**](SearchKnowledgeBaseResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_knowledge_base

> models::UpdateKnowledgeBaseResponseContent update_knowledge_base(knowledge_base_id, update_knowledge_base_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** |  | [required] |
**update_knowledge_base_request_content** | Option<[**UpdateKnowledgeBaseRequestContent**](UpdateKnowledgeBaseRequestContent.md)> |  |  |

### Return type

[**models::UpdateKnowledgeBaseResponseContent**](UpdateKnowledgeBaseResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

