# \KnowledgeBaseApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**knowledge_base_controller_create**](KnowledgeBaseApi.md#knowledge_base_controller_create) | **POST** /knowledge-base | Create Knowledge Base
[**knowledge_base_controller_find_all**](KnowledgeBaseApi.md#knowledge_base_controller_find_all) | **GET** /knowledge-base | List Knowledge Bases
[**knowledge_base_controller_find_one**](KnowledgeBaseApi.md#knowledge_base_controller_find_one) | **GET** /knowledge-base/{id} | Get Knowledge Base
[**knowledge_base_controller_remove**](KnowledgeBaseApi.md#knowledge_base_controller_remove) | **DELETE** /knowledge-base/{id} | Delete Knowledge Base
[**knowledge_base_controller_update**](KnowledgeBaseApi.md#knowledge_base_controller_update) | **PATCH** /knowledge-base/{id} | Update Knowledge Base



## knowledge_base_controller_create

> models::InlineResponse2012 knowledge_base_controller_create(knowledgebase_body)
Create Knowledge Base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledgebase_body** | [**KnowledgebaseBody**](KnowledgebaseBody.md) |  | [required] |

### Return type

[**models::InlineResponse2012**](inline_response_201_2.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## knowledge_base_controller_find_all

> Vec<models::KnowledgeBaseControllerFindAll200ResponseInner> knowledge_base_controller_find_all(limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
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

[**Vec<models::KnowledgeBaseControllerFindAll200ResponseInner>**](KnowledgeBaseController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## knowledge_base_controller_find_one

> models::InlineResponse2012 knowledge_base_controller_find_one(id)
Get Knowledge Base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::InlineResponse2012**](inline_response_201_2.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## knowledge_base_controller_remove

> models::InlineResponse2012 knowledge_base_controller_remove(id)
Delete Knowledge Base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::InlineResponse2012**](inline_response_201_2.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## knowledge_base_controller_update

> models::InlineResponse2012 knowledge_base_controller_update(id, knowledgebase_id_body)
Update Knowledge Base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**knowledgebase_id_body** | [**KnowledgebaseIdBody**](KnowledgebaseIdBody.md) |  | [required] |

### Return type

[**models::InlineResponse2012**](inline_response_201_2.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

