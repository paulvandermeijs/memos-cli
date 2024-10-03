use anyhow::Result;
use inquire::{required, Password, Text};
use url::Url;

use crate::auth::Auth;

pub fn login(auth: Auth, url: Option<String>) -> Result<()> {
    let url = url.unwrap_or_else(|| {
        Text::new("Server URL:")
            .with_validator(required!())
            .prompt()
            .unwrap()
    });
    let url = Url::parse(&url)?;

    let mut token_url = url.clone();

    token_url.set_path("/setting");

    println!("Create an access token at {token_url}");

    let token = Password::new("Access token:")
        .without_confirmation()
        .with_validator(required!())
        .prompt()
        .unwrap();

    auth.set_url(Some(url)).set_token(Some(token)).write()?;

    println!("Successfully logged in!");

    Ok(())
}
