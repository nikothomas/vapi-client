# StopSpeakingPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**num_words** | Option<**f64**> | This is the number of words that the customer has to say before the assistant will stop talking.  Words like \"stop\", \"actually\", \"no\", etc. will always interrupt immediately regardless of this value.  Words like \"okay\", \"yeah\", \"right\" will never interrupt.  When set to 0, `voiceSeconds` is used in addition to the transcriptions to determine the customer has started speaking.  Defaults to 0.  @default 0 | [optional]
**voice_seconds** | Option<**f64**> | This is the seconds customer has to speak before the assistant stops talking. This uses the VAD (Voice Activity Detection) spike to determine if the customer has started speaking.  Considerations: - A lower value might be more responsive but could potentially pick up non-speech sounds. - A higher value reduces false positives but might slightly delay the detection of speech onset.  This is only used if `numWords` is set to 0.  Defaults to 0.2  @default 0.2 | [optional]
**backoff_seconds** | Option<**f64**> | This is the seconds to wait before the assistant will start talking again after being interrupted.  Defaults to 1.  @default 1 | [optional]
**acknowledgement_phrases** | Option<**Vec<String>**> | These are the phrases that will never interrupt the assistant, even if numWords threshold is met. These are typically acknowledgement or backchanneling phrases. | [optional][default to ["i understand","i see","i got it","i hear you","im listening","im with you","right","okay","ok","sure","alright","got it","understood","yeah","yes","uh-huh","mm-hmm","gotcha","mhmm","ah","yeah okay","yeah sure"]]
**interruption_phrases** | Option<**Vec<String>**> | These are the phrases that will always interrupt the assistant immediately, regardless of numWords. These are typically phrases indicating disagreement or desire to stop. | [optional][default to ["stop","shut","up","enough","quiet","silence","but","dont","not","no","hold","wait","cut","pause","nope","nah","nevermind","never","bad","actually"]]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


