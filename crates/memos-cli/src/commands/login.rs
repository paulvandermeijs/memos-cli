use anyhow::{Error, Result};
use inquire::validator::Validation;
use inquire::{required, Password, Text};
use memos_api::apis::auth_service_api::auth_service_get_auth_status;
use url::Url;

use crate::auth::Auth;

pub(crate) fn login(auth: Auth, url: Option<String>) -> Result<()> {
    let url_validator = |input: &str| match Url::parse(input) {
        Ok(_) => Ok(Validation::Valid),
        Err(_) => Ok(Validation::Invalid("Provide a valid URL.".into())),
    };
    let url = url.unwrap_or_else(|| {
        Text::new("Server URL:")
            .with_help_message("Provide the URL to your server")
            .with_validator(required!())
            .with_validator(url_validator)
            .prompt()
            .unwrap()
    });
    let Ok(url) = Url::parse(&url) else {
        return Err(Error::msg("Provide a valid URL."));
    };

    let mut token_url = url.clone();

    token_url.set_path("/setting");

    let token = Password::new("Access token:")
        .without_confirmation()
        .with_help_message(&format!("Create an access token at {token_url}"))
        .with_validator(required!())
        .prompt()
        .unwrap();

    let auth = auth.set_url(Some(url)).set_token(Some(token));

    if let Err(_) = auth_service_get_auth_status(&auth.clone().try_into()?) {
        return Err(Error::msg("Failed to validate credentials."));
    }

    auth.write()?;

    println!("Successfully logged in!");

    Ok(())
}
