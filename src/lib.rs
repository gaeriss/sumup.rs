#[warn(warnings)]
pub mod errors;

mod api;
mod config;
mod services;

pub use errors::*;

use api::Api;
use config::Config;

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessToken {
    value: Option<String>,
    r#type: String,
    /** The number of seconds the access token will be valid. */
    expires_in: Option<u32>,
    scope: Vec<crate::config::Scope>,
    pub(crate) refresh_token: Option<String>,
}

pub struct SumUp {
    access_token: AccessToken,
    api: Api,
    config: Config,
}

impl SumUp {
    pub fn new(app_id: &str, app_secret: &str, code: &str) -> Result<Self> {
        let config = Config::new(app_id, app_secret, code);

        Self::from(config)
    }

    pub fn from(config: Config) -> Result<Self> {
        let api = Api::new();
        let authorization = services::Authorization::new(&api, &config);
        let access_token = authorization.token()?;

        let sumup = Self {
            access_token,
            api,
            config,
        };

        Ok(sumup)
    }

    pub fn refresh_token(&mut self, refresh_token: Option<&str>) -> crate::Result {
        let refresh_token = refresh_token
            .or(self.access_token.refresh_token.as_deref())
            .ok_or(crate::Error::Auth("There is no refresh token"))?;
        self.access_token = self.authorization().refresh_token(&refresh_token)?;

        Ok(())
    }

    pub fn authorization(&self) -> crate::services::Authorization {
        services::Authorization::new(&self.api, &self.config)
    }
}
