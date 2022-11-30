#[warn(warnings)]
pub mod errors;

mod api;
mod config;
mod services;

pub use errors::*;

use api::Api;
use config::Config;

#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    /** The number of seconds the access token will be valid. */
    expires_in: Option<u32>,
    scope: crate::config::Scopes,
    pub(crate) refresh_token: Option<String>,
}

impl AccessToken {
    pub fn bearer(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    account: AccountInfo,
    personal_profile: PersonalProfile,
    merchant_profile: Profile,
    requirements: Vec<String>,
    verifications: Vec<String>,
    is_migrated_payleven_br: bool,
    signup_time: String,
    details_submitted: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountInfo {
    username: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonalProfile {
    first_name: String,
    last_name: String,
    date_of_birth: String,
    mobile_phone: Option<String>,
    address: Address,
    national_id: String,
    complete: bool,
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
#[serde(deny_unknown_fields)]
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    id: String,
    transaction_code: String,
    amount: f32,
    currency: String,
    timestamp: String,
    status: Status,
    payment_type: PaymentType,
    installments_count: u32,
    merchant_code: String,
    vat_amount: f32,
    tip_amount: f32,
    entry_mode: String,
    auth_code: String,
    internal_id: String,
    product_summary: String,
    payouts_total: f32,
    payouts_received: f32,
    payout_plan: String,
    username: String,
    lat: f32,
    long: f32,
    horizontal_accuracy: f32,
    simple_payment_type: PaymentType,
    verification_method: String,
    card: TransactionCard,
    local_time: String,
    payout_type: String,
    products: Vec<Product>,
    vat_rates: Vec<f32>,
    transaction_events: Vec<TransactionEvent>,
    simple_status: String,
    links: Vec<Link>,
    events: Vec<Event>,
    location: Location,
    tax_enabled: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransactionCard {
    last_4_digits: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Product {
    name: String,
    price: f32,
    vat_rate: f32,
    single_vat_amount: f32,
    price_with_vat: f32,
    vat_amount: f32,
    quantity: f32,
    total_price: f32,
    total_with_vat: f32,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransactionEvent {
    id: u32,
    event_type: String,
    status: String,
    amount: f32,
    due_date: String,
    date: String,
    installment_number: u32,
    timestamp: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    rel: String,
    href: String,
    #[serde(rename = "type")]
    ty: String,
    min_amount: Option<f32>,
    max_amount: Option<f32>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    id: u32,
    transaction_id: String,
    #[serde(rename = "type")]
    ty: String,
    status: String,
    amount: f32,
    timestamp: String,
    fee_amount: f32,
    installment_number: u32,
    deducted_amount: f32,
    deducted_fee_amount: f32,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    lat: f32,
    long: f32,
    horizontal_accuracy: f32,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum Status {
    Successful,
    Cancelled,
    Failed,
    Refunded,
    ChargeBack,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Successful => "SUCCESSFUL",
            Status::Cancelled => "CANCELLED",
            Status::Failed => "FAILED",
            Status::Refunded => "REFUNDED",
            Status::ChargeBack => "CHARGE_BACK",
        }
        .to_string()
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum PaymentType {
    Cash,
    Pos,
    Ecom,
    Balance,
    Moto,
    Boleto,
    Unknown,
}

impl ToString for PaymentType {
    fn to_string(&self) -> String {
        match self {
            PaymentType::Cash => "CASH",
            PaymentType::Pos => "POS",
            PaymentType::Ecom => "ECOM",
            PaymentType::Balance => "BALANCE",
            PaymentType::Moto => "MOTO",
            PaymentType::Boleto => "BOLETO",
            PaymentType::Unknown => "UNKNOWN",
        }
        .to_string()
    }
}

#[derive(Clone, Copy, Debug)]
enum Type {
    Payment,
    Refund,
    ChargeBack,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Payment => "PAYMENT",
            Type::Refund => "REFUND",
            Type::ChargeBack => "CHARGE_BACK",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    transaction_data: Transaction,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaymentMethod {
    id: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubAccount {
    account_type: String,
    created_at: String,
    disabled: bool,
    id: String,
    permissions: Permissions,
    updated_at: String,
    username: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Permissions {
    create_moto_payments: bool,
    create_referral: bool,
    full_transaction_history_view: bool,
    refund_transactions: bool,
}

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
    pub fn account(&self) -> crate::services::Account {
        services::Account::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/authorization/>
     */
    pub fn authorization(&self) -> crate::services::Authorization {
        services::Authorization::new(&self.api, &self.config)
    }

    /**
     * <https://developer.sumup.com/docs/api/checkouts/>
     */
    pub fn checkout(&self) -> crate::services::Checkout {
        services::Checkout::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/customers/>
     */
    pub fn customer(&self) -> crate::services::Customer {
        services::Customer::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/merchant-account/>
     */
    pub fn merchant(&self) -> crate::services::Merchant {
        services::Merchant::new(&self.api, &self.access_token)
    }

    pub fn payouts(&self) -> crate::services::Payouts {
        services::Payouts::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/personal-account/>
     */
    pub fn personal(&self) -> crate::services::Personal {
        services::Personal::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/subaccounts/>
     */
    pub fn subaccounts(&self) -> crate::services::Subaccounts {
        services::Subaccounts::new(&self.api, &self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/transactions/>
     */
    pub fn transactions(&self) -> crate::services::Transactions {
        services::Transactions::new(&self.api, &self.access_token)
    }
}

#[cfg(test)]
mod test {
    static INIT: std::sync::Once = std::sync::Once::new();

    pub(crate) fn api() -> crate::Result<crate::SumUp> {
        INIT.call_once(|| {
            dotenv::dotenv().ok();
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
