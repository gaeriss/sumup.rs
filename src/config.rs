#[derive(Default)]
pub struct Config {
    /**
     * This is the client id that you receive after you
     * [register](https://developer.sumup.com/docs/register-app) your application in SumUp
     */
    app_id: String,
    /** This is the client secret that corresponds to the client id */
    app_secret: String,
    /** This indicates which authorization flow should be used to acquire OAuth token */
    grant_type: GrantType,
    /**
     * This is an array with all the [authorization
     * scopes](https://developer.sumup.com/docs/authorization#authorization-scopes) that you need
     * for your application
     */
    scopes: Vec<Scope>,
    /**
     * This is the code returned at the last step from [authorization code
     * flow](https://developer.sumup.com/docs/authorization#authorization-flows)
     */
    code: Option<String>,
    /** This is your SumUp's username if you want to use password authorization flow */
    username: Option<String>,
    /** This is your SumUp's password if you want to use password authorization flow */
    password: Option<String>,
    /**
     * This is the value of a valid access token that is acquired through other methods. It is used
     * if you don't want to request new access token
     */
    access_token: Option<String>,
    /** This is the refresh token through which can be requested new access token */
    refresh_token: Option<String>,
}

impl Config {
    pub fn new(app_id: &str, app_secret: &str, code: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            code: Some(code.to_string()),

            ..Default::default()
        }
    }
}

#[derive(Default)]
pub enum GrantType {
    #[default]
    AuthorizationCode,
    ClientCredentials,
    Password,
}

impl ToString for GrantType {
    fn to_string(&self) -> String {
        match self {
            Self::AuthorizationCode => "authorization_code",
            Self::ClientCredentials => "client_credentials",
            Self::Password => "password",
        }.to_string()
    }
}

pub enum Scope {
    Payments,
    TransactionsHistory,
    UserAppSettings,
    UserProfileReadonly,
    UserProfile,
    UserSubaccounts,
    UserPayoutSettings,
    Balance,
    Products,
}

impl ToString for Scope {
    fn to_string(&self) -> String {
        match self {
            Self::Payments => "payments",
            Self::TransactionsHistory => "transactions.history",
            Self::UserAppSettings => "user.app-settings",
            Self::UserProfileReadonly => "user.profile_readonly",
            Self::UserProfile => "user.profile",
            Self::UserSubaccounts => "user.subaccounts",
            Self::UserPayoutSettings => "user.payout-settings",
            Self::Balance => "balance",
            Self::Products => "products",
        }.to_string()
    }
}
