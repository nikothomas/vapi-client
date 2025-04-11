# \AnalyticsApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analytics_get**](AnalyticsApi.md#analytics_get) | **POST** /analytics | Create Analytics Queries



## analytics_get

> Vec<models::AnalyticsQueryResult> analytics_get(analytics_body)
Create Analytics Queries

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_body** | [**AnalyticsBody**](AnalyticsBody.md) |  | [required] |

### Return type

[**Vec<models::AnalyticsQueryResult>**](AnalyticsQueryResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

