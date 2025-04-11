# \WorkflowApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workflow_workflow_controller_create**](WorkflowApi.md#workflow_workflow_controller_create) | **POST** /workflow | Create Workflow
[**workflow_workflow_controller_delete**](WorkflowApi.md#workflow_workflow_controller_delete) | **DELETE** /workflow/{id} | Delete Workflow
[**workflow_workflow_controller_find_all**](WorkflowApi.md#workflow_workflow_controller_find_all) | **GET** /workflow | Get Workflows
[**workflow_workflow_controller_find_one**](WorkflowApi.md#workflow_workflow_controller_find_one) | **GET** /workflow/{id} | Get Workflow
[**workflow_workflow_controller_update**](WorkflowApi.md#workflow_workflow_controller_update) | **PATCH** /workflow/{id} | Update Workflow



## workflow_workflow_controller_create

> models::Workflow workflow_workflow_controller_create(create_workflow_dto)
Create Workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_workflow_dto** | [**CreateWorkflowDto**](CreateWorkflowDto.md) |  | [required] |

### Return type

[**models::Workflow**](Workflow.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workflow_workflow_controller_delete

> models::Workflow workflow_workflow_controller_delete(id)
Delete Workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Workflow**](Workflow.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workflow_workflow_controller_find_all

> Vec<models::Workflow> workflow_workflow_controller_find_all()
Get Workflows

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Workflow>**](Workflow.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workflow_workflow_controller_find_one

> models::Workflow workflow_workflow_controller_find_one(id)
Get Workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Workflow**](Workflow.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workflow_workflow_controller_update

> models::Workflow workflow_workflow_controller_update(id, workflow_id_body)
Update Workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**workflow_id_body** | [**WorkflowIdBody**](WorkflowIdBody.md) |  | [required] |

### Return type

[**models::Workflow**](Workflow.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

