# \TestSuitesApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**test_suite_controller_create**](TestSuitesApi.md#test_suite_controller_create) | **POST** /test-suite | Create Test Suite
[**test_suite_controller_find_all_paginated**](TestSuitesApi.md#test_suite_controller_find_all_paginated) | **GET** /test-suite | List Test Suites
[**test_suite_controller_find_one**](TestSuitesApi.md#test_suite_controller_find_one) | **GET** /test-suite/{id} | Get Test Suite
[**test_suite_controller_remove**](TestSuitesApi.md#test_suite_controller_remove) | **DELETE** /test-suite/{id} | Delete Test Suite
[**test_suite_controller_update**](TestSuitesApi.md#test_suite_controller_update) | **PATCH** /test-suite/{id} | Update Test Suite



## test_suite_controller_create

> models::TestSuite test_suite_controller_create(create_test_suite_dto)
Create Test Suite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_test_suite_dto** | [**CreateTestSuiteDto**](CreateTestSuiteDto.md) |  | [required] |

### Return type

[**models::TestSuite**](TestSuite.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_controller_find_all_paginated

> models::TestSuitesPaginatedResponse test_suite_controller_find_all_paginated(page, sort_order, limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
List Test Suites

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> | This is the page number to return. Defaults to 1. |  |
**sort_order** | Option<**String**> | This is the sort order for pagination. Defaults to 'DESC'. |  |
**limit** | Option<**f64**> | This is the maximum number of items to return. Defaults to 100. |  |
**created_at_gt** | Option<**String**> | This will return items where the createdAt is greater than the specified value. |  |
**created_at_lt** | Option<**String**> | This will return items where the createdAt is less than the specified value. |  |
**created_at_ge** | Option<**String**> | This will return items where the createdAt is greater than or equal to the specified value. |  |
**created_at_le** | Option<**String**> | This will return items where the createdAt is less than or equal to the specified value. |  |
**updated_at_gt** | Option<**String**> | This will return items where the updatedAt is greater than the specified value. |  |
**updated_at_lt** | Option<**String**> | This will return items where the updatedAt is less than the specified value. |  |
**updated_at_ge** | Option<**String**> | This will return items where the updatedAt is greater than or equal to the specified value. |  |
**updated_at_le** | Option<**String**> | This will return items where the updatedAt is less than or equal to the specified value. |  |

### Return type

[**models::TestSuitesPaginatedResponse**](TestSuitesPaginatedResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_controller_find_one

> models::TestSuite test_suite_controller_find_one(id)
Get Test Suite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TestSuite**](TestSuite.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_controller_remove

> models::TestSuite test_suite_controller_remove(id)
Delete Test Suite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TestSuite**](TestSuite.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_controller_update

> models::TestSuite test_suite_controller_update(id, update_test_suite_dto)
Update Test Suite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_test_suite_dto** | [**UpdateTestSuiteDto**](UpdateTestSuiteDto.md) |  | [required] |

### Return type

[**models::TestSuite**](TestSuite.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

