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
pub struct TableNodeRow {
    #[serde(rename = "cells", skip_serializing_if = "Option::is_none")]
    pub cells: Option<Vec<models::V1Node>>,
}

impl TableNodeRow {
    pub fn new() -> TableNodeRow {
        TableNodeRow {
            cells: None,
        }
    }
}

