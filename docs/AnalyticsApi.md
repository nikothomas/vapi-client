# \AnalyticsApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analytics_controller_query**](AnalyticsApi.md#analytics_controller_query) | **POST** /analytics | Create Analytics Queries



## analytics_controller_query

> Vec<models::AnalyticsQueryResult> analytics_controller_query(analytics_query_dto)
Create Analytics Queries

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_query_dto** | [**AnalyticsQueryDto**](AnalyticsQueryDto.md) |  | [required] |

### Return type

[**Vec<models::AnalyticsQueryResult>**](AnalyticsQueryResult.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

