use std::fs;

use anyhow::{Error, Result};
use memos_api::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Auth {
    url: Option<Url>,
    token: Option<String>,
}

impl Auth {
    pub fn read() -> Self {
        let auth_data = std::fs::read_to_string(auth_path());

        if let Err(_) = auth_data {
            return Default::default();
        }

        let auth: Auth = toml::from_str(&auth_data.unwrap()).unwrap();

        auth
    }

    pub fn write(&self) -> Result<()> {
        let auth_data = toml::to_string(self).unwrap();
        let auth_path = auth_path();
        let auth_path_dir = std::path::Path::new(&auth_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();

        if let Err(_) = fs::metadata(auth_path_dir) {
            fs::create_dir_all(auth_path_dir)?;
        }

        std::fs::write(&auth_path, auth_data).map_err(|_| Error::msg("Couldn't write auth."))
    }

    pub(crate) fn set_url(mut self, url: Option<Url>) -> Self {
        self.url = url;

        self
    }

    pub(crate) fn set_token(mut self, token: Option<String>) -> Self {
        self.token = token;

        self
    }
}

impl TryInto<Configuration> for Auth {
    type Error = anyhow::Error;

    fn try_into(self) -> std::prelude::v1::Result<Configuration, Self::Error> {
        let Some(url) = self.url else {
            return Err(Error::msg("Missing server URL. Login using `memo login`."));
        };
        let Some(token) = self.token else {
            return Err(Error::msg(
                "Missing access token. Login using `memo login`.",
            ));
        };

        let base_path = url.to_string().trim_end_matches('/').to_string();
        let bearer_access_token = Some(token);
        let configuration = Configuration {
            base_path,
            bearer_access_token,
            ..Default::default()
        };

        Ok(configuration)
    }
}

fn auth_path() -> String {
    let home = std::env::var("HOME").unwrap();

    format!("{home}/.config/memos-cli/auth.toml")
}
