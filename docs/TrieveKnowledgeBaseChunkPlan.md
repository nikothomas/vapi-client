# TrieveKnowledgeBaseChunkPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_ids** | Option<**Vec<String>**> | These are the file ids that will be used to create the vector store. To upload files, use the `POST /files` endpoint. | [optional]
**websites** | Option<**Vec<String>**> | These are the websites that will be used to create the vector store. | [optional]
**target_splits_per_chunk** | Option<**f64**> | This is an optional field which allows you to specify the number of splits you want per chunk. If not specified, the default 20 is used. However, you may want to use a different number. | [optional]
**split_delimiters** | Option<**Vec<String>**> | This is an optional field which allows you to specify the delimiters to use when splitting the file before chunking the text. If not specified, the default [.!?\\n] are used to split into sentences. However, you may want to use spaces or other delimiters. | [optional]
**rebalance_chunks** | Option<**bool**> | This is an optional field which allows you to specify whether or not to rebalance the chunks created from the file. If not specified, the default true is used. If true, Trieve will evenly distribute remainder splits across chunks such that 66 splits with a target_splits_per_chunk of 20 will result in 3 chunks with 22 splits each. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


