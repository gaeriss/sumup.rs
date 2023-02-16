#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    /** The number of seconds the access token will be valid. */
    pub expires_in: Option<u32>,
    pub scope: crate::config::Scopes,
    pub refresh_token: Option<String>,
}

impl AccessToken {
    #[must_use]
    pub fn bearer(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Account {
    pub account: AccountInfo,
    pub personal_profile: PersonalProfile,
    pub merchant_profile: Profile,
    pub requirements: Vec<String>,
    pub verifications: Vec<String>,
    pub is_migrated_payleven_br: bool,
    pub signup_time: String,
    pub details_submitted: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AccountInfo {
    pub username: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PersonalProfile {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub mobile_phone: Option<String>,
    pub address: Address,
    pub national_id: String,
    pub complete: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Checkout {
    pub amount: f32,
    pub currency: String,
    pub checkout_reference: String,
    pub merchant_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_to_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_from_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Customer {
    pub customer_id: String,
    #[serde(default)]
    pub personal_detail: Details,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Details {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postalcode: Option<String>,
    pub post_code: Option<String>,
    pub landline: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Profile {
    pub merchant_code: String,
    pub company_name: String,
    pub legal_type: LegalType,
    pub merchant_category_code: String,
    pub address: Address,
    pub business_owners: Vec<()>,
    pub doing_business_as: DoingBusinessAs,
    pub locale: String,
    pub complete: bool,
    pub extdev: bool,
    pub country: String,
    pub default_currency: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LegalType {
    pub id: u32,
    pub full_description: String,
    pub description: String,
    pub sole_trader: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DoingBusinessAs {
    pub business_name: String,
    pub email: String,
    pub dynamic_descriptor: String,
    #[serde(skip_serializing)]
    pub address: Address,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct BankAccount {
    pub bank_code: String,
    pub account_number: String,
    pub account_holder_name: String,
    pub status: String,
    pub primary: bool,
    pub created_at: String,
    pub bank_name: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Settings {
    pub tax_enabled: bool,
    pub payout_type: String,
    pub payout_period: String,
    pub payout_on_demand_available: bool,
    pub payout_on_demand: bool,
    pub printers_enabled: bool,
    pub payout_instrument: String,
    pub moto_payment: String,
    pub checkout_payments: String,
    pub daily_payout_email: bool,
    pub monthly_payout_email: bool,
    pub gross_settlement: bool,
    pub bank_account_change_blocked: bool,
    pub operator_personal_profile_name: bool,
    pub operator_personal_profile_date_of_birth: bool,
    pub operator_personal_profile_address: bool,
    pub operator_personal_profile_note: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Card {
    pub name: String,
    pub number: String,
    pub expiry_year: String,
    pub expiry_month: String,
    pub cvv: String,
    pub zip_code: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Payout {
    pub amount: f32,
    pub currency: String,
    pub date: String,
    pub fee: f32,
    pub id: u32,
    pub reference: String,
    pub status: String,
    pub transaction_code: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Transaction {
    pub id: String,
    pub transaction_code: String,
    pub amount: f32,
    pub currency: String,
    pub timestamp: String,
    pub status: Status,
    pub payment_type: PaymentType,
    pub installments_count: u32,
    pub merchant_code: String,
    pub vat_amount: f32,
    pub tip_amount: f32,
    pub entry_mode: String,
    pub auth_code: String,
    pub internal_id: String,
    pub product_summary: String,
    pub payouts_total: f32,
    pub payouts_received: f32,
    pub payout_plan: String,
    pub username: String,
    pub lat: f32,
    pub long: f32,
    pub horizontal_accuracy: f32,
    pub simple_payment_type: PaymentType,
    pub verification_method: String,
    pub card: TransactionCard,
    pub local_time: String,
    pub payout_type: String,
    pub products: Vec<Product>,
    pub vat_rates: Vec<f32>,
    pub transaction_events: Vec<TransactionEvent>,
    pub simple_status: String,
    pub links: Vec<Link>,
    pub events: Vec<Event>,
    pub location: Location,
    pub tax_enabled: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct TransactionCard {
    pub last_4_digits: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub vat_rate: f32,
    pub single_vat_amount: f32,
    pub price_with_vat: f32,
    pub vat_amount: f32,
    pub quantity: f32,
    pub total_price: f32,
    pub total_with_vat: f32,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct TransactionEvent {
    pub id: u32,
    pub event_type: String,
    pub status: String,
    pub amount: f32,
    pub due_date: String,
    pub date: String,
    pub installment_number: u32,
    pub timestamp: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Link {
    pub rel: String,
    pub href: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub min_amount: Option<f32>,
    pub max_amount: Option<f32>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Event {
    pub id: u32,
    pub transaction_id: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub status: String,
    pub amount: f32,
    pub timestamp: String,
    pub fee_amount: f32,
    pub installment_number: u32,
    pub deducted_amount: f32,
    pub deducted_fee_amount: f32,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Location {
    pub lat: f32,
    pub long: f32,
    pub horizontal_accuracy: f32,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
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
pub enum PaymentType {
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
pub enum Type {
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
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Receipt {
    pub transaction_data: Transaction,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PaymentMethod {
    pub id: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SubAccount {
    pub account_type: String,
    pub created_at: String,
    pub disabled: bool,
    pub id: String,
    pub permissions: Permissions,
    pub updated_at: String,
    pub username: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Permissions {
    pub create_moto_payments: bool,
    pub create_referral: bool,
    pub full_transaction_history_view: bool,
    pub refund_transactions: bool,
}
