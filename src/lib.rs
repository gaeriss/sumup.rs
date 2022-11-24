#[warn(warnings)]
pub mod errors;

mod api;
mod authorization;
mod config;

pub use errors::*;

use api::Api;
use authorization::Authorization;
use config::Config;

pub struct SumUp {
    access_token: authorization::AccessToken,
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
        let authorization = Authorization::new(&api, &config);
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

    pub fn authorization(&self) -> crate::Authorization {
        Authorization::new(&self.api, &self.config)
    }
}
