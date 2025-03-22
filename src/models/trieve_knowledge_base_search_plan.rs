/*
 * Vapi API
 *
 * API for building voice assistants
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct TrieveKnowledgeBaseSearchPlan {
    /// Specifies the number of top chunks to return. This corresponds to the `page_size` parameter in Trieve.
    #[serde(rename = "topK", skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f64>,
    /// If true, stop words (specified in server/src/stop-words.txt in the git repo) will be removed. This will preserve queries that are entirely stop words.
    #[serde(rename = "removeStopWords", skip_serializing_if = "Option::is_none")]
    pub remove_stop_words: Option<bool>,
    /// This is the score threshold to filter out chunks with a score below the threshold for cosine distance metric. For Manhattan Distance, Euclidean Distance, and Dot Product, it will filter out scores above the threshold distance. This threshold applies before weight and bias modifications. If not specified, this defaults to no threshold. A threshold of 0 will default to no threshold.
    #[serde(rename = "scoreThreshold", skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
    /// This is the search method used when searching for relevant chunks from the vector store.
    #[serde(rename = "searchType")]
    pub search_type: SearchType,
}

impl TrieveKnowledgeBaseSearchPlan {
    pub fn new(search_type: SearchType) -> TrieveKnowledgeBaseSearchPlan {
        TrieveKnowledgeBaseSearchPlan {
            top_k: None,
            remove_stop_words: None,
            score_threshold: None,
            search_type,
        }
    }
}
/// This is the search method used when searching for relevant chunks from the vector store.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum SearchType {
    #[serde(rename = "fulltext")]
    Fulltext,
    #[serde(rename = "semantic")]
    Semantic,
    #[serde(rename = "hybrid")]
    Hybrid,
    #[serde(rename = "bm25")]
    Bm25,
}

impl Default for SearchType {
    fn default() -> SearchType {
        Self::Fulltext
    }
}
