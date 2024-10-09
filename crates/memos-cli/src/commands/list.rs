use anyhow::Result;
use memos_api::apis::{
    auth_service_api::auth_service_get_auth_status, memo_service_api::memo_service_list_memos,
};

use crate::auth::Auth;

pub(crate) fn list(auth: Auth) -> Result<()> {
    let configuration = auth.try_into()?;
    let auth_status = auth_service_get_auth_status(&configuration)?;
    let filter = format!(
        "creator == '{}' && row_status == 'NORMAL'",
        auth_status.name.unwrap()
    );
    let result = memo_service_list_memos(&configuration, None, None, Some(&filter))?;
    let output = result
        .memos
        .unwrap()
        .into_iter()
        .map(|memo| memo.content.unwrap())
        .collect::<Vec<String>>()
        .join("\n\n---\n\n");

    println!("{output}");

    Ok(())
}
