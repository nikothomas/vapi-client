# \KnowledgeBasesApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**knowledge_bases_create**](KnowledgeBasesApi.md#knowledge_bases_create) | **POST** /knowledge-base | Create Knowledge Base
[**knowledge_bases_delete**](KnowledgeBasesApi.md#knowledge_bases_delete) | **DELETE** /knowledge-base/{id} | Delete Knowledge Base
[**knowledge_bases_get**](KnowledgeBasesApi.md#knowledge_bases_get) | **GET** /knowledge-base/{id} | Get Knowledge Base
[**knowledge_bases_list**](KnowledgeBasesApi.md#knowledge_bases_list) | **GET** /knowledge-base | List Knowledge Bases
[**knowledge_bases_update**](KnowledgeBasesApi.md#knowledge_bases_update) | **PATCH** /knowledge-base/{id} | Update Knowledge Base



## knowledge_bases_create

> models::KnowledgeBasesCreateResponse knowledge_bases_create(knowledge_bases_create_request)
Create Knowledge Base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_bases_create_request** | [**KnowledgeBasesCreateRequest**](KnowledgeBasesCreateRequest.md) |  | [required] |

### Return type

[**models::KnowledgeBasesCreateResponse**](KnowledgeBasesCreateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## knowledge_bases_delete

> models::KnowledgeBasesDeleteResponse knowledge_bases_delete(id)
Delete Knowledge Base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::KnowledgeBasesDeleteResponse**](KnowledgeBasesDeleteResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## knowledge_bases_get

> models::KnowledgeBasesGetResponse knowledge_bases_get(id)
Get Knowledge Base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::KnowledgeBasesGetResponse**](KnowledgeBasesGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## knowledge_bases_list

> Vec<models::KnowledgeBasesListResponseItem> knowledge_bases_list(limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
List Knowledge Bases

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

[**Vec<models::KnowledgeBasesListResponseItem>**](KnowledgeBasesListResponseItem.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## knowledge_bases_update

> models::KnowledgeBasesUpdateResponse knowledge_bases_update(id, knowledge_bases_update_request)
Update Knowledge Base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**knowledge_bases_update_request** | [**KnowledgeBasesUpdateRequest**](KnowledgeBasesUpdateRequest.md) |  | [required] |

### Return type

[**models::KnowledgeBasesUpdateResponse**](KnowledgeBasesUpdateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

