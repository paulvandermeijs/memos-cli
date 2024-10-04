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
pub struct V1Memo {
    /// The name of the memo. Format: memos/{id} id is the system generated id.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user defined id of the memo.
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "rowStatus", skip_serializing_if = "Option::is_none")]
    pub row_status: Option<models::V1RowStatus>,
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(rename = "displayTime", skip_serializing_if = "Option::is_none")]
    pub display_time: Option<String>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<models::V1Node>>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<models::V1Visibility>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "pinned", skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<models::V1Resource>>,
    #[serde(rename = "relations", skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<models::V1MemoRelation>>,
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<models::V1Reaction>>,
    #[serde(rename = "property", skip_serializing_if = "Option::is_none")]
    pub property: Option<Box<models::V1MemoProperty>>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// The snippet of the memo content. Plain text only.
    #[serde(rename = "snippet", skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

impl V1Memo {
    pub fn new() -> V1Memo {
        V1Memo {
            name: None,
            uid: None,
            row_status: None,
            creator: None,
            create_time: None,
            update_time: None,
            display_time: None,
            content: None,
            nodes: None,
            visibility: None,
            tags: None,
            pinned: None,
            resources: None,
            relations: None,
            reactions: None,
            property: None,
            parent: None,
            snippet: None,
        }
    }
}

