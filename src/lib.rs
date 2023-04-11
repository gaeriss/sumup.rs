#[warn(warnings)]
pub mod config;
pub mod errors;
pub mod services;

mod api;
mod entity;

pub use config::Config;
pub use entity::*;
pub use errors::*;

use api::Api;

#[derive(Clone, Debug)]
pub struct SumUp {
    access_token: AccessToken,
    api: Api,
    config: Config,
}

impl SumUp {
    pub fn new(client_id: &str, client_secret: &str, code: &str) -> Result<Self> {
        let config = Config::new(client_id, client_secret, code);

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

    #[must_use]
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    /**
     * <https://developer.sumup.com/docs/api/generate-a-token/>
     */
    pub fn refresh_token(&mut self, refresh_token: Option<&str>) -> crate::Result {
        let refresh_token = refresh_token
            .or(self.access_token.refresh_token.as_deref())
            .ok_or(crate::Error::Auth("There is no refresh token"))?;
        self.access_token = self.authorization().refresh_token(refresh_token)?;

        Ok(())
    }

    /**
     * <https://developer.sumup.com/docs/api/account-details/>
     */
    #[must_use]
    pub fn account(&self) -> crate::services::Account {
        services::Account::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/authorization/>
     */
    #[must_use]
    pub fn authorization(&self) -> crate::services::Authorization {
        services::Authorization::new(&self.api, &self.config)
    }

    /**
     * <https://developer.sumup.com/docs/api/checkouts/>
     */
    #[must_use]
    pub fn checkout(&self) -> crate::services::Checkout {
        services::Checkout::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/customers/>
     */
    #[must_use]
    pub fn customer(&self) -> crate::services::Customer {
        services::Customer::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/merchant-account/>
     */
    #[must_use]
    pub fn merchant(&self) -> crate::services::Merchant {
        services::Merchant::new(&self.api, &self.access_token)
    }

    #[must_use]
    pub fn payouts(&self) -> crate::services::Payouts {
        services::Payouts::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/personal-account/>
     */
    #[must_use]
    pub fn personal(&self) -> crate::services::Personal {
        services::Personal::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/subaccounts/>
     */
    #[must_use]
    pub fn subaccounts(&self) -> crate::services::Subaccounts {
        services::Subaccounts::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/transactions/>
     */
    #[must_use]
    pub fn transactions(&self) -> crate::services::Transactions {
        services::Transactions::new(&self.api, &self.access_token)
    }
}

#[cfg(test)]
mod test {
    static INIT: std::sync::Once = std::sync::Once::new();

    pub(crate) fn api() -> crate::Result<crate::SumUp> {
        INIT.call_once(|| {
            dotenvy::dotenv().ok();
            env_logger::init();
        });

        let config = crate::Config {
            client_id: std::env::var("CLIENT_ID").unwrap(),
            client_secret: std::env::var("CLIENT_SECRET").unwrap(),
            username: std::env::var("USERNAME").ok(),
            password: std::env::var("PASSWORD").ok(),
            grant_type: crate::config::GrantType::Password,

            ..Default::default()
        };

        crate::SumUp::from(config)
    }

    #[test]
    fn new() -> crate::Result {
        api().map(|_| ())
    }

    #[test]
    fn refresh_token() -> crate::Result {
        let mut api = api()?;
        let access_token = api.access_token.clone();

        api.refresh_token(None)?;

        assert_ne!(access_token, api.access_token);

        Ok(())
    }
}
