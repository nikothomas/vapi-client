# LivekitSmartEndpointingPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | [**models::LivekitSmartEndpointingPlanProvider**](LivekitSmartEndpointingPlanProvider.md) |  | 
**wait_function** | Option<**String**> | This expression describes how long the bot will wait to start speaking based on the likelihood that the user has reached an endpoint.  This is a millisecond valued function. It maps probabilities (real numbers on [0,1]) to milliseconds that the bot should wait before speaking ([0, \\infty]). Any negative values that are returned are set to zero (the bot can't start talking in the past).  A probability of zero represents very high confidence that the caller has stopped speaking, and would like the bot to speak to them. A probability of one represents very high confidence that the caller is still speaking.  Under the hood, this is parsed into a mathjs expression. Whatever you use to write your expression needs to be valid with respect to mathjs  @default \"70 + 4000 * x\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


