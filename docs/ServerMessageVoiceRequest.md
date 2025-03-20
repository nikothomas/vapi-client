# ServerMessageVoiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageAssistantRequestPhoneNumber**](ServerMessageAssistantRequest_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"voice-request\" is sent when using `assistant.voice={ \"type\": \"custom-voice\" }`.  Here is what the request will look like:  POST https://{assistant.voice.server.url} Content-Type: application/json  {   \"messsage\": {     \"type\": \"voice-request\",     \"text\": \"Hello, world!\",     \"sampleRate\": 24000,     ...other metadata about the call...   } }  The expected response is 1-channel 16-bit raw PCM audio at the sample rate specified in the request. Here is how the response will be piped to the transport: ``` response.on('data', (chunk: Buffer) => {   outputStream.write(chunk); }); ``` | 
**timestamp** | Option<**f64**> | This is the ISO-8601 formatted timestamp of when the message was sent. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> | This is a live version of the `call.artifact`.  This matches what is stored on `call.artifact` after the call. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that is currently active. This is provided for convenience.  This matches one of the following: - `call.assistant`, - `call.assistantId`, - `call.squad[n].assistant`, - `call.squad[n].assistantId`, - `call.squadId->[n].assistant`, - `call.squadId->[n].assistantId`. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer associated with the call.  This matches one of the following: - `call.customer`, - `call.customerId`. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call object.  This matches what was returned in POST /call.  Note: This might get stale during the call. To get the latest call object, especially after the call is ended, use GET /call/:id. | [optional]
**text** | **String** | This is the text to be synthesized. | 
**sample_rate** | **f64** | This is the sample rate to be synthesized. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


