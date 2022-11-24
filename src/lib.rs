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

impl AccessToken {
    pub fn bearer(&self) -> String {
        format!("Bearer {}", self.value.as_deref().unwrap_or_default())
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Checkout {
    amount: f32,
    currency: String,
    checkout_reference: String,
    merchant_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_to_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_from_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
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
        self.access_token = self.authorization().refresh_token(refresh_token)?;

        Ok(())
    }

    pub fn authorization(&self) -> crate::services::Authorization {
        services::Authorization::new(&self.api, &self.config)
    }

    pub fn checkout(&self) -> crate::services::Checkout {
        services::Checkout::new(&self.api, &self.access_token)
    }
}
