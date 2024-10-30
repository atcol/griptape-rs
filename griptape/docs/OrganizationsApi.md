# \OrganizationsApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /organizations/{organization_id} | 
[**list_organizations**](OrganizationsApi.md#list_organizations) | **GET** /organizations | 



## get_organization

> models::GetOrganizationResponseContent get_organization(organization_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::GetOrganizationResponseContent**](GetOrganizationResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organizations

> models::ListOrganizationsResponseContent list_organizations()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListOrganizationsResponseContent**](ListOrganizationsResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

