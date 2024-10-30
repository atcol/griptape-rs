# \RulesetsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ruleset**](RulesetsApi.md#create_ruleset) | **POST** /rulesets | 
[**delete_ruleset**](RulesetsApi.md#delete_ruleset) | **DELETE** /rulesets/{ruleset_id} | 
[**get_ruleset**](RulesetsApi.md#get_ruleset) | **GET** /rulesets/{ruleset_id} | 
[**list_rulesets**](RulesetsApi.md#list_rulesets) | **GET** /rulesets | 
[**update_ruleset**](RulesetsApi.md#update_ruleset) | **PATCH** /rulesets/{ruleset_id} | 



## create_ruleset

> models::CreateRulesetResponseContent create_ruleset(create_ruleset_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ruleset_request_content** | [**CreateRulesetRequestContent**](CreateRulesetRequestContent.md) |  | [required] |

### Return type

[**models::CreateRulesetResponseContent**](CreateRulesetResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ruleset

> delete_ruleset(ruleset_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ruleset_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ruleset

> models::GetRulesetResponseContent get_ruleset(ruleset_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ruleset_id** | **String** |  | [required] |

### Return type

[**models::GetRulesetResponseContent**](GetRulesetResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rulesets

> models::ListRulesetsResponseContent list_rulesets(page, page_size, alias)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |
**page_size** | Option<**f64**> |  |  |
**alias** | Option<**String**> |  |  |

### Return type

[**models::ListRulesetsResponseContent**](ListRulesetsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ruleset

> models::UpdateRulesetResponseContent update_ruleset(ruleset_id, update_ruleset_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ruleset_id** | **String** |  | [required] |
**update_ruleset_request_content** | Option<[**UpdateRulesetRequestContent**](UpdateRulesetRequestContent.md)> |  |  |

### Return type

[**models::UpdateRulesetResponseContent**](UpdateRulesetResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

