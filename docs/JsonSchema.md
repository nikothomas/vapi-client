# JsonSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::JsonSchemaType**](JsonSchemaType.md) |  | 
**items** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is required if the type is \"array\". This is the schema of the items in the array.  This is of type JsonSchema. However, Swagger doesn't support circular references. | [optional]
**properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is required if the type is \"object\". This specifies the properties of the object.  This is a map of string to JsonSchema. However, Swagger doesn't support circular references. | [optional]
**description** | Option<**String**> | This is the description to help the model understand what it needs to output. | [optional]
**required** | Option<**Vec<String>**> | This is a list of properties that are required.  This only makes sense if the type is \"object\". | [optional]
**regex** | Option<**String**> | This is a regex that will be used to validate data in question. | [optional]
**value** | Option<**String**> | This the value that will be used in filling the property. | [optional]
**target** | Option<**String**> | This the target variable that will be filled with the value of this property. | [optional]
**r#enum** | Option<**Vec<String>**> | This array specifies the allowed values that can be used to restrict the output of the model. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


