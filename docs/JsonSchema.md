# JsonSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of output you'd like.  `string`, `number`, `integer`, `boolean` are the primitive types and should be obvious.  `array` and `object` are more interesting and quite powerful. They allow you to define nested structures.  For `array`, you can define the schema of the items in the array using the `items` property.  For `object`, you can define the properties of the object using the `properties` property. | 
**items** | Option<[**serde_json::Value**](.md)> | This is required if the type is \"array\". This is the schema of the items in the array.  This is of type JsonSchema. However, Swagger doesn't support circular references. | [optional]
**properties** | Option<[**serde_json::Value**](.md)> | This is required if the type is \"object\". This specifies the properties of the object.  This is a map of string to JsonSchema. However, Swagger doesn't support circular references. | [optional]
**description** | Option<**String**> | This is the description to help the model understand what it needs to output. | [optional]
**required** | Option<**Vec<String>**> | This is a list of properties that are required.  This only makes sense if the type is \"object\". | [optional]
**value** | Option<**String**> | This the value that will be used in filling the property. | [optional]
**target** | Option<**String**> | This the target variable that will be filled with the value of this property. | [optional]
**r#enum** | Option<**Vec<String>**> | This array specifies the allowed values that can be used to restrict the output of the model. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


