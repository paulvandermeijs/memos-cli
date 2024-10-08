/*
 * Memos API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ParseMarkdownResponse {
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<models::V1Node>>,
}

impl V1ParseMarkdownResponse {
    pub fn new() -> V1ParseMarkdownResponse {
        V1ParseMarkdownResponse {
            nodes: None,
        }
    }
}

