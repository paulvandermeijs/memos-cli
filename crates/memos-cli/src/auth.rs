use std::fs;

use anyhow::{Error, Result};
use memos_api::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Auth {
    url: Option<Url>,
    token: Option<String>,
}

impl Auth {
    pub fn read() -> Auth {
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

        std::fs::write(&auth_path, auth_data).map_err(|_| Error::msg("Couldn't write auth"))
    }

    pub(crate) fn get_url(&self) -> &Option<Url> {
        &self.url
    }

    pub(crate) fn set_url(mut self, url: Option<Url>) -> Self {
        self.url = url;

        self
    }
    pub(crate) fn get_token(&self) -> &Option<String> {
        &self.token
    }

    pub(crate) fn set_token(mut self, token: Option<String>) -> Self {
        self.token = token;

        self
    }
}

impl Into<Configuration> for &Auth {
    fn into(self) -> Configuration {
        let base_path = self
            .get_url()
            .clone()
            .unwrap()
            .to_string()
            .trim_end_matches('/')
            .to_string();
        let bearer_access_token = Some(self.get_token().clone().unwrap());
        let configuration = Configuration {
            base_path,
            bearer_access_token,
            ..Default::default()
        };

        configuration
    }
}

fn auth_path() -> String {
    let home = std::env::var("HOME").unwrap();

    format!("{home}/.config/memos-cli/auth.toml")
}
