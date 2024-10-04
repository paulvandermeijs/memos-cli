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
pub struct TheIdentityProviderToUpdate {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::Apiv1IdentityProviderType>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "identifierFilter", skip_serializing_if = "Option::is_none")]
    pub identifier_filter: Option<String>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::Apiv1IdentityProviderConfig>>,
}

impl TheIdentityProviderToUpdate {
    pub fn new() -> TheIdentityProviderToUpdate {
        TheIdentityProviderToUpdate {
            r#type: None,
            title: None,
            identifier_filter: None,
            config: None,
        }
    }
}

