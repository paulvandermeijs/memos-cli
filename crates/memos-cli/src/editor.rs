use anyhow::Result;
use std::{io::Write, process::Command};

pub(crate) fn get_content_from_editor(input: Option<String>) -> Result<String> {
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
