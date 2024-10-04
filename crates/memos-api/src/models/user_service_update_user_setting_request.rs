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
pub struct UserServiceUpdateUserSettingRequest {
    /// The preferred locale of the user.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// The preferred appearance of the user.
    #[serde(rename = "appearance", skip_serializing_if = "Option::is_none")]
    pub appearance: Option<String>,
    /// The default visibility of the memo.
    #[serde(rename = "memoVisibility", skip_serializing_if = "Option::is_none")]
    pub memo_visibility: Option<String>,
}

impl UserServiceUpdateUserSettingRequest {
    pub fn new() -> UserServiceUpdateUserSettingRequest {
        UserServiceUpdateUserSettingRequest {
            locale: None,
            appearance: None,
            memo_visibility: None,
        }
    }
}

