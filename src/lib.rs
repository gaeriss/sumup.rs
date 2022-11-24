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

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Customer {
    customer_id: String,
    #[serde(default)]
    personal_detail: Details,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Details {
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postalcode: Option<String>,
    post_code: Option<String>,
    landline: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    merchant_code: String,
    company_name: String,
    legal_type: LegalType,
    merchant_category_code: String,
    address: Address,
    business_owners: Vec<()>,
    doing_business_as: DoingBusinessAs,
    locale: String,
    complete: bool,
    extdev: bool,
    country: String,
    default_currency: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegalType {
    id: u32,
    full_description: String,
    description: String,
    sole_trader: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DoingBusinessAs {
    business_name: String,
    email: String,
    dynamic_descriptor: String,
    #[serde(skip_serializing)]
    address: Address,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankAccount {
    bank_code: String,
    account_number: String,
    account_holder_name: String,
    status: String,
    primary: bool,
    // @TODO chrono::DateTime
    created_at: String,
    bank_name: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    tax_enabled: bool,
    payout_type: String,
    payout_period: String,
    payout_on_demand_available: bool,
    payout_on_demand: bool,
    printers_enabled: bool,
    payout_instrument: String,
    moto_payment: String,
    checkout_payments: String,
    daily_payout_email: bool,
    monthly_payout_email: bool,
    gross_settlement: bool,
    bank_account_change_blocked: bool,
    operator_personal_profile_name: bool,
    operator_personal_profile_date_of_birth: bool,
    operator_personal_profile_address: bool,
    operator_personal_profile_note: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Card {
    name: String,
    number: String,
    expiry_year: String,
    expiry_month: String,
    cvv: String,
    zip_code: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Payout {
    amount: f32,
    currency: String,
    date: String,
    fee: f32,
    id: u32,
    reference: String,
    status: String,
    transaction_code: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Transaction {}

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

    pub fn customer(&self) -> crate::services::Customer {
        services::Customer::new(&self.api, &self.access_token)
    }

    pub fn merchant(&self) -> crate::services::Merchant {
        services::Merchant::new(&self.api, &self.access_token)
    }

    pub fn payouts(&self) -> crate::services::Payouts {
        services::Payouts::new(&self.api, &self.access_token)
    }
}
