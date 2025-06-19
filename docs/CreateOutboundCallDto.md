# CreateOutboundCallDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customers** | Option<[**Vec<models::CreateCustomerDto>**](CreateCustomerDTO.md)> | This is used to issue batch calls to multiple customers.  Only relevant for `outboundPhoneCall`. To call a single customer, use `customer` instead. | [optional]
**name** | Option<**String**> | This is the name of the call. This is just for your own reference. | [optional]
**schedule_plan** | Option<[**models::SchedulePlan**](SchedulePlan.md)> | This is the schedule plan of the call. | [optional]
**transport** | Option<[**serde_json::Value**](.md)> | This is the transport of the call. | [optional]
**assistant_id** | Option<**String**> | This is the assistant ID that will be used for the call. To use a transient assistant, use `assistant` instead.  To start a call with: - Assistant, use `assistantId` or `assistant` - Squad, use `squadId` or `squad` - Workflow, use `workflowId` or `workflow` | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that will be used for the call. To use an existing assistant, use `assistantId` instead.  To start a call with: - Assistant, use `assistant` - Squad, use `squad` - Workflow, use `workflow` | [optional]
**assistant_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> | These are the overrides for the `assistant` or `assistantId`'s settings and template variables. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for the call. To use a transient squad, use `squad` instead.  To start a call with: - Assistant, use `assistant` or `assistantId` - Squad, use `squad` or `squadId` - Workflow, use `workflow` or `workflowId` | [optional]
**squad** | Option<[**models::CreateSquadDto**](CreateSquadDTO.md)> | This is a squad that will be used for the call. To use an existing squad, use `squadId` instead.  To start a call with: - Assistant, use `assistant` or `assistantId` - Squad, use `squad` or `squadId` - Workflow, use `workflow` or `workflowId` | [optional]
**workflow_id** | Option<**String**> | This is the workflow that will be used for the call. To use a transient workflow, use `workflow` instead.  To start a call with: - Assistant, use `assistant` or `assistantId` - Squad, use `squad` or `squadId` - Workflow, use `workflow` or `workflowId` | [optional]
**workflow** | Option<[**models::CreateWorkflowDto**](CreateWorkflowDTO.md)> | This is a workflow that will be used for the call. To use an existing workflow, use `workflowId` instead.  To start a call with: - Assistant, use `assistant` or `assistantId` - Squad, use `squad` or `squadId` - Workflow, use `workflow` or `workflowId` | [optional]
**workflow_overrides** | Option<[**models::WorkflowOverrides**](WorkflowOverrides.md)> | These are the overrides for the `workflow` or `workflowId`'s settings and template variables. | [optional]
**phone_number_id** | Option<**String**> | This is the phone number that will be used for the call. To use a transient number, use `phoneNumber` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**phone_number** | Option<[**models::ImportTwilioPhoneNumberDto**](ImportTwilioPhoneNumberDTO.md)> | This is the phone number that will be used for the call. To use an existing number, use `phoneNumberId` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**customer_id** | Option<**String**> | This is the customer that will be called. To call a transient customer , use `customer` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer that will be called. To call an existing customer, use `customerId` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


