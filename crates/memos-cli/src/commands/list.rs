use anyhow::Result;
use log::info;
use memos_api::apis::{
    auth_service_api::auth_service_get_auth_status, memo_service_api::memo_service_list_memos,
};
use memos_cli_ui::list::ListView;

use crate::auth::Auth;

pub(crate) fn list(auth: Auth) -> Result<()> {
    let configuration = auth.try_into()?;
    let auth_status = auth_service_get_auth_status(&configuration)?;
    let filter = format!(
        "creator == '{}' && row_status == 'NORMAL'",
        auth_status.name.unwrap()
    );

    info!("Using filter {filter:?}");

    let result = memo_service_list_memos(&configuration, None, None, Some(&filter))?;
    let memos = &result.memos.unwrap();
    let list_view = ListView::try_new(memos)?;

    list_view.draw()?;

    Ok(())
}
