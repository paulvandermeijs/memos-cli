use anyhow::Result;
use memos_api::apis::memo_service_api::memo_service_create_memo;
use memos_api::models::{V1CreateMemoRequest, V1Visibility};

use crate::auth::Auth;
use crate::editor::get_content_from_editor;
use crate::io::get_stdin;

pub(crate) fn create(auth: &Auth, no_edit: bool, workspace: bool, public: bool) -> Result<()> {
    let input = get_stdin();
    let content = if !no_edit {
        get_content_from_editor(input)?
    } else {
        input.unwrap_or_else(|| "".to_string())
    };
    let content = content.trim().to_string();

    if content.is_empty() {
        return Err(anyhow::Error::msg("Memo can not be empty"));
    }

    let content = Some(content);
    let visibility = if workspace {
        V1Visibility::Protected
    } else if public {
        V1Visibility::Public
    } else {
        V1Visibility::Protected
    };
    let visibility = Some(visibility);
    let body = V1CreateMemoRequest {
        content,
        visibility,
        ..Default::default()
    };

    memo_service_create_memo(&auth.into(), body)?;

    Ok(())
}
