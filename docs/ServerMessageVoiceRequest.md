# ServerMessageVoiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageVoiceRequestPhoneNumber**](ServerMessageVoiceRequestPhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"voice-request\" is sent when using `assistant.voice={ \"type\": \"custom-voice\" }`.  Here is what the request will look like:  POST https://{assistant.voice.server.url} Content-Type: application/json  {   \"messsage\": {     \"type\": \"voice-request\",     \"text\": \"Hello, world!\",     \"sampleRate\": 24000,     ...other metadata about the call...   } }  The expected response is 1-channel 16-bit raw PCM audio at the sample rate specified in the request. Here is how the response will be piped to the transport: ``` response.on('data', (chunk: Buffer) => {   outputStream.write(chunk); }); ``` | 
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]
**text** | **String** | This is the text to be synthesized. | 
**sample_rate** | **f64** | This is the sample rate to be synthesized. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


