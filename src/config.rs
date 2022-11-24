#[derive(Clone, Debug, Default)]
pub struct Config {
    /**
     * This is the client id that you receive after you
     * [register](https://developer.sumup.com/docs/register-app) your application in SumUp
     */
    pub client_id: String,
    /** This is the client secret that corresponds to the client id */
    pub client_secret: String,
    /** This indicates which authorization flow should be used to acquire OAuth token */
    pub grant_type: GrantType,
    /**
     * This is an array with all the [authorization
     * scopes](https://developer.sumup.com/docs/authorization#authorization-scopes) that you need
     * for your application
     */
    pub scopes: Vec<Scope>,
    /**
     * This is the code returned at the last step from [authorization code
     * flow](https://developer.sumup.com/docs/authorization#authorization-flows)
     */
    pub code: Option<String>,
    /** This is your SumUp's username if you want to use password authorization flow */
    pub username: Option<String>,
    /** This is your SumUp's password if you want to use password authorization flow */
    pub password: Option<String>,
    /**
     * This is the value of a valid access token that is acquired through other methods. It is used
     * if you don't want to request new access token
     */
    pub access_token: Option<String>,
    /** This is the refresh token through which can be requested new access token */
    pub refresh_token: Option<String>,
}

impl Config {
    pub fn new(client_id: &str, client_secret: &str, code: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            code: Some(code.to_string()),

            ..Default::default()
        }
    }

    pub fn scopes(&self) -> crate::Result<String> {
        let scopes = self
            .scopes
            .iter()
            .map(serde_json::to_string)
            .collect::<serde_json::Result<Vec<_>>>()?
            .join(" ");

        Ok(scopes)
    }
}

#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub enum GrantType {
    #[default]
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "client_credentials")]
    ClientCredentials,
    #[serde(rename = "password")]
    Password,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Scope {
    #[serde(rename = "payments")]
    Payments,
    #[serde(rename = "transactions.history")]
    TransactionsHistory,
    #[serde(rename = "user.app-settings")]
    UserAppSettings,
    #[serde(rename = "user.profile_readonly")]
    UserProfileReadonly,
    #[serde(rename = "user.profile")]
    UserProfile,
    #[serde(rename = "user.subaccounts")]
    UserSubaccounts,
    #[serde(rename = "user.payout-settings")]
    UserPayoutSettings,
    #[serde(rename = "balance")]
    Balance,
    #[serde(rename = "products")]
    Products,
}
