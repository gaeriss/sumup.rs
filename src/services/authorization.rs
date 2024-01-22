#[derive(Clone, Debug)]
pub struct Authorization<'a> {
    api: &'a crate::Api,
    config: &'a crate::Config,
}

impl<'a> Authorization<'a> {
    #[must_use]
    pub fn new(api: &'a crate::Api, config: &'a crate::Config) -> Self {
        Self { api, config }
    }

    pub fn token(&self) -> crate::Result<crate::AccessToken> {
        let token = if let Some(access_token) = &self.config.access_token {
            crate::AccessToken {
                access_token: access_token.clone(),
                scope: self.config.scopes.clone(),
                refresh_token: self.config.refresh_token.clone(),
                expires_in: Some(0),

                ..Default::default()
            }
        } else if let Some(refresh_token) = &self.config.refresh_token {
            crate::AccessToken {
                scope: self.config.scopes.clone(),
                refresh_token: Some(refresh_token.clone()),
                expires_in: Some(0),

                ..Default::default()
            }
        } else {
            use crate::config::GrantType::*;

            match self.config.grant_type {
                AuthorizationCode => self.token_by_code()?,
                ClientCredentials => self.token_by_client_credentials()?,
                Password => self.token_password()?,
            }
        };

        Ok(token)
    }

    fn token_by_code(&self) -> crate::Result<crate::AccessToken> {
        let payload = ureq::json!({
            "grant_type": "authorization_code",
            "client_id": self.config.client_id,
            "client_secret": self.config.client_secret,
            "code": self.config.code,
        });

        self.api.token(payload)
    }

    fn token_by_client_credentials(&self) -> crate::Result<crate::AccessToken> {
        let payload = ureq::json!({
            "grant_type": "client_credentials",
            "client_id": self.config.client_id,
            "client_secret": self.config.client_secret,
            "scope": self.config.scopes,
        });

        self.api.token(payload)
    }

    fn token_password(&self) -> crate::Result<crate::AccessToken> {
        let username = self
            .config
            .username
            .as_ref()
            .ok_or(crate::Error::Auth("Missing username configuration"))?;

        let password = self
            .config
            .password
            .as_ref()
            .ok_or(crate::Error::Auth("Missing password configuration"))?;

        let payload = ureq::json!({
            "grant_type": "password",
            "client_id": self.config.client_id,
            "client_secret": self.config.client_secret,
            "scope": self.config.scopes,
            "username": username,
            "password": password,
        });

        self.api.token(payload)
    }

    pub(crate) fn refresh_token(&self, refresh_token: &str) -> crate::Result<crate::AccessToken> {
        let payload = ureq::json!({
            "grant_type": "refresh_token",
            "client_id": self.config.client_id,
            "client_secret": self.config.client_secret,
            "scope": self.config.scopes,
            "refresh_token": refresh_token,
        });

        self.api.token(payload)
    }
}
