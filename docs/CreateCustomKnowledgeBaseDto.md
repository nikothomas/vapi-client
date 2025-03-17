# CreateCustomKnowledgeBaseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This knowledge base is bring your own knowledge base implementation. | 
**server** | [**models::Server**](Server.md) | This is where the knowledge base request will be sent.  Request Example:  POST https://{server.url} Content-Type: application/json  {   \"messsage\": {     \"type\": \"knowledge-base-request\",     \"messages\": [       {         \"role\": \"user\",         \"content\": \"Why is ocean blue?\"       }     ],     ...other metadata about the call...   } }  Response Expected: ``` {   \"message\": {      \"role\": \"assistant\",      \"content\": \"The ocean is blue because water absorbs everything but blue.\",   }, // YOU CAN RETURN THE EXACT RESPONSE TO SPEAK   \"documents\": [     {       \"content\": \"The ocean is blue primarily because water absorbs colors in the red part of the light spectrum and scatters the blue light, making it more visible to our eyes.\",       \"similarity\": 1     },     {       \"content\": \"Blue light is scattered more by the water molecules than other colors, enhancing the blue appearance of the ocean.\",       \"similarity\": .5     }   ] // OR, YOU CAN RETURN AN ARRAY OF DOCUMENTS THAT WILL BE SENT TO THE MODEL } ``` | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


