# \BlocksApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**block_controller_create**](BlocksApi.md#block_controller_create) | **POST** /block | Create Block
[**block_controller_find_all**](BlocksApi.md#block_controller_find_all) | **GET** /block | List Blocks
[**block_controller_find_one**](BlocksApi.md#block_controller_find_one) | **GET** /block/{id} | Get Block
[**block_controller_remove**](BlocksApi.md#block_controller_remove) | **DELETE** /block/{id} | Delete Block
[**block_controller_update**](BlocksApi.md#block_controller_update) | **PATCH** /block/{id} | Update Block



## block_controller_create

> models::BlockControllerFindAll200ResponseInner block_controller_create(block_controller_create_request)
Create Block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_controller_create_request** | [**BlockControllerCreateRequest**](BlockControllerCreateRequest.md) |  | [required] |

### Return type

[**models::BlockControllerFindAll200ResponseInner**](BlockController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_controller_find_all

> Vec<models::BlockControllerFindAll200ResponseInner> block_controller_find_all(limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
List Blocks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

[**Vec<models::BlockControllerFindAll200ResponseInner>**](BlockController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_controller_find_one

> models::BlockControllerFindAll200ResponseInner block_controller_find_one(id)
Get Block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::BlockControllerFindAll200ResponseInner**](BlockController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_controller_remove

> models::BlockControllerFindAll200ResponseInner block_controller_remove(id)
Delete Block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::BlockControllerFindAll200ResponseInner**](BlockController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_controller_update

> models::BlockControllerFindAll200ResponseInner block_controller_update(id, block_controller_update_request)
Update Block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**block_controller_update_request** | [**BlockControllerUpdateRequest**](BlockControllerUpdateRequest.md) |  | [required] |

### Return type

[**models::BlockControllerFindAll200ResponseInner**](BlockController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

