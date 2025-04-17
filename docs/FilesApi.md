# \FilesApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**file_controller_create**](FilesApi.md#file_controller_create) | **POST** /file | Upload File
[**file_controller_find_all**](FilesApi.md#file_controller_find_all) | **GET** /file | List Files
[**file_controller_find_one**](FilesApi.md#file_controller_find_one) | **GET** /file/{id} | Get File
[**file_controller_remove**](FilesApi.md#file_controller_remove) | **DELETE** /file/{id} | Delete File
[**file_controller_update**](FilesApi.md#file_controller_update) | **PATCH** /file/{id} | Update File



## file_controller_create

> models::File file_controller_create(file)
Upload File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | This is the File you want to upload for use with the Knowledge Base. | [required] |

### Return type

[**models::File**](File.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_controller_find_all

> Vec<models::File> file_controller_find_all()
List Files

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::File>**](File.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_controller_find_one

> models::File file_controller_find_one(id)
Get File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::File**](File.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_controller_remove

> models::File file_controller_remove(id)
Delete File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::File**](File.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_controller_update

> models::File file_controller_update(id, update_file_dto)
Update File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_file_dto** | [**UpdateFileDto**](UpdateFileDto.md) |  | [required] |

### Return type

[**models::File**](File.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

