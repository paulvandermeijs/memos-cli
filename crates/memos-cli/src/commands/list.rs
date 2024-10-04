use anyhow::Result;
use memos_api::apis::{configuration::Configuration, memo_service_api::memo_service_list_memos};

use crate::auth::Auth;

pub(crate) fn list(auth: &Auth) -> Result<()> {
    let base_path = auth
        .get_url()
        .clone()
        .unwrap()
        .to_string()
        .trim_end_matches('/')
        .to_string();
    let bearer_access_token = Some(auth.get_token().clone().unwrap());
    let configuration = Configuration {
        base_path,
        bearer_access_token,
        ..Default::default()
    };
    let result = memo_service_list_memos(&configuration, None, None, None)?;

    Ok(())
}
