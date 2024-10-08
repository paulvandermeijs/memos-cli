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
pub struct MemoServiceUpsertMemoReactionBody {
    #[serde(rename = "reaction", skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Box<models::V1Reaction>>,
}

impl MemoServiceUpsertMemoReactionBody {
    pub fn new() -> MemoServiceUpsertMemoReactionBody {
        MemoServiceUpsertMemoReactionBody {
            reaction: None,
        }
    }
}

