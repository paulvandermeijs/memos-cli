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
pub struct Apiv1WorkspaceGeneralSetting {
    /// disallow_user_registration disallows user registration.
    #[serde(rename = "disallowUserRegistration", skip_serializing_if = "Option::is_none")]
    pub disallow_user_registration: Option<bool>,
    /// disallow_password_auth disallows password authentication.
    #[serde(rename = "disallowPasswordAuth", skip_serializing_if = "Option::is_none")]
    pub disallow_password_auth: Option<bool>,
    /// additional_script is the additional script.
    #[serde(rename = "additionalScript", skip_serializing_if = "Option::is_none")]
    pub additional_script: Option<String>,
    /// additional_style is the additional style.
    #[serde(rename = "additionalStyle", skip_serializing_if = "Option::is_none")]
    pub additional_style: Option<String>,
    #[serde(rename = "customProfile", skip_serializing_if = "Option::is_none")]
    pub custom_profile: Option<Box<models::Apiv1WorkspaceCustomProfile>>,
    /// week_start_day_offset is the week start day offset from Sunday. 0: Sunday, 1: Monday, 2: Tuesday, 3: Wednesday, 4: Thursday, 5: Friday, 6: Saturday Default is Sunday.
    #[serde(rename = "weekStartDayOffset", skip_serializing_if = "Option::is_none")]
    pub week_start_day_offset: Option<i32>,
    /// disallow_change_username disallows changing username.
    #[serde(rename = "disallowChangeUsername", skip_serializing_if = "Option::is_none")]
    pub disallow_change_username: Option<bool>,
    /// disallow_change_nickname disallows changing nickname.
    #[serde(rename = "disallowChangeNickname", skip_serializing_if = "Option::is_none")]
    pub disallow_change_nickname: Option<bool>,
}

impl Apiv1WorkspaceGeneralSetting {
    pub fn new() -> Apiv1WorkspaceGeneralSetting {
        Apiv1WorkspaceGeneralSetting {
            disallow_user_registration: None,
            disallow_password_auth: None,
            additional_script: None,
            additional_style: None,
            custom_profile: None,
            week_start_day_offset: None,
            disallow_change_username: None,
            disallow_change_nickname: None,
        }
    }
}

