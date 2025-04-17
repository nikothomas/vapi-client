# \PhoneNumbersApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**phone_number_controller_create**](PhoneNumbersApi.md#phone_number_controller_create) | **POST** /phone-number | Create Phone Number
[**phone_number_controller_find_all**](PhoneNumbersApi.md#phone_number_controller_find_all) | **GET** /phone-number | List Phone Numbers
[**phone_number_controller_find_one**](PhoneNumbersApi.md#phone_number_controller_find_one) | **GET** /phone-number/{id} | Get Phone Number
[**phone_number_controller_remove**](PhoneNumbersApi.md#phone_number_controller_remove) | **DELETE** /phone-number/{id} | Delete Phone Number
[**phone_number_controller_update**](PhoneNumbersApi.md#phone_number_controller_update) | **PATCH** /phone-number/{id} | Update Phone Number



## phone_number_controller_create

> models::PhoneNumber phone_number_controller_create(phonenumber_body)
Create Phone Number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phonenumber_body** | [**PhonenumberBody**](PhonenumberBody.md) |  | [required] |

### Return type

[**models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## phone_number_controller_find_all

> Vec<models::PhoneNumber1> phone_number_controller_find_all(limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
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

[**Vec<models::PhoneNumber1>**](PhoneNumber_1.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## phone_number_controller_find_one

> models::PhoneNumber phone_number_controller_find_one(id)
Get Phone Number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## phone_number_controller_remove

> models::PhoneNumber phone_number_controller_remove(id)
Delete Phone Number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## phone_number_controller_update

> models::PhoneNumber phone_number_controller_update(id, phonenumber_id_body)
Update Phone Number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**phonenumber_id_body** | [**PhonenumberIdBody**](PhonenumberIdBody.md) |  | [required] |

### Return type

[**models::PhoneNumber**](PhoneNumber.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

