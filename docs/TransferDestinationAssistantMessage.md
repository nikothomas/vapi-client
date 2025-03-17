# TransferDestinationAssistantMessage

## Enum Variants

| Name | Description |
|---- | -----|
| CustomMessage | This is spoken to the customer before connecting them to the destination.  Usage: - If this is not provided and transfer tool messages is not provided, default is \&quot;Transferring the call now\&quot;. - If set to \&quot;\&quot;, nothing is spoken. This is useful when you want to silently transfer. This is especially useful when transferring between assistants in a squad. In this scenario, you likely also want to set &#x60;assistant.firstMessageMode&#x3D;assistant-speaks-first-with-model-generated-message&#x60; for the destination assistant.  This accepts a string or a ToolMessageStart class. Latter is useful if you want to specify multiple messages for different languages through the &#x60;contents&#x60; field. |
| String | This is spoken to the customer before connecting them to the destination.  Usage: - If this is not provided and transfer tool messages is not provided, default is \&quot;Transferring the call now\&quot;. - If set to \&quot;\&quot;, nothing is spoken. This is useful when you want to silently transfer. This is especially useful when transferring between assistants in a squad. In this scenario, you likely also want to set &#x60;assistant.firstMessageMode&#x3D;assistant-speaks-first-with-model-generated-message&#x60; for the destination assistant.  This accepts a string or a ToolMessageStart class. Latter is useful if you want to specify multiple messages for different languages through the &#x60;contents&#x60; field. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


