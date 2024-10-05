use anyhow::Result;
use memos_api::apis::memo_service_api::memo_service_list_memos;

use crate::auth::Auth;

pub(crate) fn list(auth: Auth) -> Result<()> {
    let result = memo_service_list_memos(&auth.try_into()?, None, None, None)?;

    Ok(())
}
