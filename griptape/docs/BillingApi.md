# \BillingApi

All URIs are relative to *https://cloud.griptape.ai/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_billing_management_url**](BillingApi.md#create_billing_management_url) | **POST** /billing/management-url | 
[**create_checkout_session**](BillingApi.md#create_checkout_session) | **POST** /billing/checkout-session | 



## create_billing_management_url

> models::CreateBillingManagementUrlResponseContent create_billing_management_url()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CreateBillingManagementUrlResponseContent**](CreateBillingManagementUrlResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_checkout_session

> models::CreateCheckoutSessionResponseContent create_checkout_session(create_checkout_session_request_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_checkout_session_request_content** | [**CreateCheckoutSessionRequestContent**](CreateCheckoutSessionRequestContent.md) |  | [required] |

### Return type

[**models::CreateCheckoutSessionResponseContent**](CreateCheckoutSessionResponseContent.md)

### Authorization

[smithy.api.httpBearerAuth](../README.md#smithy.api.httpBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

