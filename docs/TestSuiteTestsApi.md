# \TestSuiteTestsApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**test_suite_tests_test_suite_test_controller_create**](TestSuiteTestsApi.md#test_suite_tests_test_suite_test_controller_create) | **POST** /test-suite/{testSuiteId}/test | Create Test
[**test_suite_tests_test_suite_test_controller_find_all_paginated**](TestSuiteTestsApi.md#test_suite_tests_test_suite_test_controller_find_all_paginated) | **GET** /test-suite/{testSuiteId}/test | List Tests
[**test_suite_tests_test_suite_test_controller_find_one**](TestSuiteTestsApi.md#test_suite_tests_test_suite_test_controller_find_one) | **GET** /test-suite/{testSuiteId}/test/{id} | Get Test
[**test_suite_tests_test_suite_test_controller_remove**](TestSuiteTestsApi.md#test_suite_tests_test_suite_test_controller_remove) | **DELETE** /test-suite/{testSuiteId}/test/{id} | Delete Test
[**test_suite_tests_test_suite_test_controller_update**](TestSuiteTestsApi.md#test_suite_tests_test_suite_test_controller_update) | **PATCH** /test-suite/{testSuiteId}/test/{id} | Update Test



## test_suite_tests_test_suite_test_controller_create

> models::TestSuiteTestControllerCreateResponse test_suite_tests_test_suite_test_controller_create(test_suite_id, test_suite_test_controller_create_request)
Create Test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**test_suite_test_controller_create_request** | [**TestSuiteTestControllerCreateRequest**](TestSuiteTestControllerCreateRequest.md) |  | [required] |

### Return type

[**models::TestSuiteTestControllerCreateResponse**](TestSuiteTestControllerCreateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_tests_test_suite_test_controller_find_all_paginated

> models::TestSuiteTestsPaginatedResponse test_suite_tests_test_suite_test_controller_find_all_paginated(test_suite_id, page, sort_order, limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
List Tests

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**page** | Option<**f64**> | This is the page number to return. Defaults to 1. |  |
**sort_order** | Option<[**TestSuiteTestControllerFindAllPaginatedRequestSortOrder**](.md)> | This is the sort order for pagination. Defaults to 'DESC'. |  |
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

[**models::TestSuiteTestsPaginatedResponse**](TestSuiteTestsPaginatedResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_tests_test_suite_test_controller_find_one

> models::TestSuiteTestControllerFindOneResponse test_suite_tests_test_suite_test_controller_find_one(test_suite_id, id)
Get Test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::TestSuiteTestControllerFindOneResponse**](TestSuiteTestControllerFindOneResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_tests_test_suite_test_controller_remove

> models::TestSuiteTestControllerRemoveResponse test_suite_tests_test_suite_test_controller_remove(test_suite_id, id)
Delete Test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::TestSuiteTestControllerRemoveResponse**](TestSuiteTestControllerRemoveResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_tests_test_suite_test_controller_update

> models::TestSuiteTestControllerUpdateResponse test_suite_tests_test_suite_test_controller_update(test_suite_id, id, test_suite_test_controller_update_request)
Update Test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**test_suite_test_controller_update_request** | [**TestSuiteTestControllerUpdateRequest**](TestSuiteTestControllerUpdateRequest.md) |  | [required] |

### Return type

[**models::TestSuiteTestControllerUpdateResponse**](TestSuiteTestControllerUpdateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

