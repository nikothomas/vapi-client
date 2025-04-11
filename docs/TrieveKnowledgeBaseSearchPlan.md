# TrieveKnowledgeBaseSearchPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**top_k** | Option<**f64**> | Specifies the number of top chunks to return. This corresponds to the `page_size` parameter in Trieve. | [optional]
**remove_stop_words** | Option<**bool**> | If true, stop words (specified in server/src/stop-words.txt in the git repo) will be removed. This will preserve queries that are entirely stop words. | [optional]
**score_threshold** | Option<**f64**> | This is the score threshold to filter out chunks with a score below the threshold for cosine distance metric. For Manhattan Distance, Euclidean Distance, and Dot Product, it will filter out scores above the threshold distance. This threshold applies before weight and bias modifications. If not specified, this defaults to no threshold. A threshold of 0 will default to no threshold. | [optional]
**search_type** | [**models::TrieveKnowledgeBaseSearchPlanSearchType**](TrieveKnowledgeBaseSearchPlanSearchType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


