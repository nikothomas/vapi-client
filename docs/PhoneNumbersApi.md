# \PhoneNumbersApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**phone_numbers_create**](PhoneNumbersApi.md#phone_numbers_create) | **POST** /phone-number | Create Phone Number
[**phone_numbers_delete**](PhoneNumbersApi.md#phone_numbers_delete) | **DELETE** /phone-number/{id} | Delete Phone Number
[**phone_numbers_get**](PhoneNumbersApi.md#phone_numbers_get) | **GET** /phone-number/{id} | Get Phone Number
[**phone_numbers_list**](PhoneNumbersApi.md#phone_numbers_list) | **GET** /phone-number | List Phone Numbers
[**phone_numbers_update**](PhoneNumbersApi.md#phone_numbers_update) | **PATCH** /phone-number/{id} | Update Phone Number



## phone_numbers_create

> models::PhoneNumbersCreateResponse phone_numbers_create(phone_numbers_create_request)
Create Phone Number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_numbers_create_request** | [**PhoneNumbersCreateRequest**](PhoneNumbersCreateRequest.md) |  | [required] |

### Return type

[**models::PhoneNumbersCreateResponse**](PhoneNumbersCreateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## phone_numbers_delete

> models::PhoneNumbersDeleteResponse phone_numbers_delete(id)
Delete Phone Number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::PhoneNumbersDeleteResponse**](PhoneNumbersDeleteResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## phone_numbers_get

> models::PhoneNumbersGetResponse phone_numbers_get(id)
Get Phone Number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::PhoneNumbersGetResponse**](PhoneNumbersGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## phone_numbers_list

> Vec<models::PhoneNumbersListResponseItem> phone_numbers_list(limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
List Phone Numbers

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

[**Vec<models::PhoneNumbersListResponseItem>**](PhoneNumbersListResponseItem.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## phone_numbers_update

> models::PhoneNumbersUpdateResponse phone_numbers_update(id, phone_numbers_update_request)
Update Phone Number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**phone_numbers_update_request** | [**PhoneNumbersUpdateRequest**](PhoneNumbersUpdateRequest.md) |  | [required] |

### Return type

[**models::PhoneNumbersUpdateResponse**](PhoneNumbersUpdateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

