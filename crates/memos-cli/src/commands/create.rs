use anyhow::Result;
use memos_api::apis::memo_service_api::memo_service_create_memo;
use memos_api::models::{V1CreateMemoRequest, V1Visibility};
use std::io::Read;
use std::io::Write;
use std::process::Command;

use crate::auth::Auth;

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

fn get_stdin() -> Option<String> {
    use std::io::IsTerminal;

    let stdin = std::io::stdin();

    if stdin.is_terminal() {
        return None;
    }

    let mut buffer = String::new();

    if let Err(_) = stdin.lock().read_to_string(&mut buffer) {
        return None;
    }

    let buffer = buffer.trim().to_string();

    if buffer.is_empty() {
        return None;
    }

    Some(buffer)
}

fn get_content_from_editor(input: Option<String>) -> Result<String> {
    let Ok(editor_command) = get_editor_command() else {
        return Err(anyhow::Error::msg("No editor configured"));
    };
    let mut tmpfile = tempfile::NamedTempFile::new()?;

    if let Some(input) = input {
        tmpfile.write_all(input.as_bytes())?;
    }

    let path = tmpfile.path();

    Command::new(&editor_command)
        .arg(path)
        .spawn()
        .or_else(|_| {
            Err(anyhow::Error::msg(format!(
                "Failed to run editor command {editor_command}"
            )))
        })?
        .wait()
        .or_else(|_| Err(anyhow::Error::msg("".to_string())))?;

    let content = std::fs::read_to_string(path)?;

    Ok(content)
}

fn get_editor_command() -> Result<String, impl std::error::Error> {
    std::env::var("VISUAL").or_else(|_| std::env::var("EDITOR"))
}
