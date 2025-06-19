# FourierDenoisingPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Whether Fourier denoising is enabled. Note that this is experimental and may not work as expected. | [optional][default to false]
**media_detection_enabled** | Option<**bool**> | Whether automatic media detection is enabled. When enabled, the filter will automatically detect consistent background TV/music/radio and switch to more aggressive filtering settings. Only applies when enabled is true. | [optional][default to true]
**static_threshold** | Option<**f64**> | Static threshold in dB used as fallback when no baseline is established. | [optional][default to -35]
**baseline_offset_db** | Option<**f64**> | How far below the rolling baseline to filter audio, in dB. Lower values (e.g., -10) are more aggressive, higher values (e.g., -20) are more conservative. | [optional][default to -15]
**window_size_ms** | Option<**f64**> | Rolling window size in milliseconds for calculating the audio baseline. Larger windows adapt more slowly but are more stable. | [optional][default to 3000]
**baseline_percentile** | Option<**f64**> | Percentile to use for baseline calculation (1-99). Higher percentiles (e.g., 85) focus on louder speech, lower percentiles (e.g., 50) include quieter speech. | [optional][default to 85]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


