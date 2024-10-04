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
pub struct V1MemoProperty {
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "hasLink", skip_serializing_if = "Option::is_none")]
    pub has_link: Option<bool>,
    #[serde(rename = "hasTaskList", skip_serializing_if = "Option::is_none")]
    pub has_task_list: Option<bool>,
    #[serde(rename = "hasCode", skip_serializing_if = "Option::is_none")]
    pub has_code: Option<bool>,
    #[serde(rename = "hasIncompleteTasks", skip_serializing_if = "Option::is_none")]
    pub has_incomplete_tasks: Option<bool>,
}

impl V1MemoProperty {
    pub fn new() -> V1MemoProperty {
        V1MemoProperty {
            tags: None,
            has_link: None,
            has_task_list: None,
            has_code: None,
            has_incomplete_tasks: None,
        }
    }
}

