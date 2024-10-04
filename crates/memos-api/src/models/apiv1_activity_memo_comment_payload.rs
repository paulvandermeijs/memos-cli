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

/// Apiv1ActivityMemoCommentPayload : ActivityMemoCommentPayload represents the payload of a memo comment activity.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Apiv1ActivityMemoCommentPayload {
    /// The memo id of comment.
    #[serde(rename = "memoId", skip_serializing_if = "Option::is_none")]
    pub memo_id: Option<i32>,
    /// The memo id of related memo.
    #[serde(rename = "relatedMemoId", skip_serializing_if = "Option::is_none")]
    pub related_memo_id: Option<i32>,
}

impl Apiv1ActivityMemoCommentPayload {
    /// ActivityMemoCommentPayload represents the payload of a memo comment activity.
    pub fn new() -> Apiv1ActivityMemoCommentPayload {
        Apiv1ActivityMemoCommentPayload {
            memo_id: None,
            related_memo_id: None,
        }
    }
}

