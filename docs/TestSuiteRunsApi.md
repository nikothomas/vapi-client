# \TestSuiteRunsApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**test_suite_runs_test_suite_run_controller_create**](TestSuiteRunsApi.md#test_suite_runs_test_suite_run_controller_create) | **POST** /test-suite/{testSuiteId}/run | Create Test Suite Run
[**test_suite_runs_test_suite_run_controller_find_all_paginated**](TestSuiteRunsApi.md#test_suite_runs_test_suite_run_controller_find_all_paginated) | **GET** /test-suite/{testSuiteId}/run | List Test Suite Runs
[**test_suite_runs_test_suite_run_controller_find_one**](TestSuiteRunsApi.md#test_suite_runs_test_suite_run_controller_find_one) | **GET** /test-suite/{testSuiteId}/run/{id} | Get Test Suite Run
[**test_suite_runs_test_suite_run_controller_remove**](TestSuiteRunsApi.md#test_suite_runs_test_suite_run_controller_remove) | **DELETE** /test-suite/{testSuiteId}/run/{id} | Delete Test Suite Run
[**test_suite_runs_test_suite_run_controller_update**](TestSuiteRunsApi.md#test_suite_runs_test_suite_run_controller_update) | **PATCH** /test-suite/{testSuiteId}/run/{id} | Update Test Suite Run



## test_suite_runs_test_suite_run_controller_create

> models::TestSuiteRun test_suite_runs_test_suite_run_controller_create(test_suite_id, test_suite_id_run_body)
Create Test Suite Run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**test_suite_id_run_body** | [**TestSuiteIdRunBody**](TestSuiteIdRunBody.md) |  | [required] |

### Return type

[**models::TestSuiteRun**](TestSuiteRun.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_runs_test_suite_run_controller_find_all_paginated

> models::TestSuiteRunsPaginatedResponse test_suite_runs_test_suite_run_controller_find_all_paginated(test_suite_id, page, sort_order, limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
List Test Suite Runs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**page** | Option<**f64**> | This is the page number to return. Defaults to 1. |  |
**sort_order** | Option<[**TestSuiteRunControllerFindAllPaginatedRequestSortOrder**](.md)> | This is the sort order for pagination. Defaults to 'DESC'. |  |
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

[**models::TestSuiteRunsPaginatedResponse**](TestSuiteRunsPaginatedResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_runs_test_suite_run_controller_find_one

> models::TestSuiteRun test_suite_runs_test_suite_run_controller_find_one(test_suite_id, id)
Get Test Suite Run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::TestSuiteRun**](TestSuiteRun.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_runs_test_suite_run_controller_remove

> models::TestSuiteRun test_suite_runs_test_suite_run_controller_remove(test_suite_id, id)
Delete Test Suite Run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::TestSuiteRun**](TestSuiteRun.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_suite_runs_test_suite_run_controller_update

> models::TestSuiteRun test_suite_runs_test_suite_run_controller_update(test_suite_id, id, run_id_body)
Update Test Suite Run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_suite_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**run_id_body** | [**RunIdBody**](RunIdBody.md) |  | [required] |

### Return type

[**models::TestSuiteRun**](TestSuiteRun.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

