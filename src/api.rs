#[derive(Clone, Debug, Default)]
pub struct Api {}

macro_rules! url {
    ($path:literal) => {
        concat!("https://api.sumup.com", $path)
    };
    ($path:literal, $( $param:expr ),+ ) => {
        &vec![
            url!($path).to_string(),
            $( $param.to_string() ),+
        ].join("/")
    };
}

impl Api {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token(&self, payload: impl serde::Serialize) -> crate::Result<crate::AccessToken> {
        Self::send(reqwest::Method::POST, url!("/token"), Some(payload), None)
    }

    pub fn account_get(&self, access_token: &crate::AccessToken) -> crate::Result<crate::Account> {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/me"),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn checkout_create(
        &self,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        Self::send(
            reqwest::Method::POST,
            url!("/checkouts"),
            Some(payload),
            Some(access_token),
        )
    }

    pub fn checkout_list(
        &self,
        checkout_reference: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Checkout>> {
        Self::send(
            reqwest::Method::GET,
            &format!(
                "{}?checkout_reference={checkout_reference}",
                url!("/v0.1/checkouts")
            ),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn checkout_get(
        &self,
        id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Checkout> {
        Self::send(
            reqwest::Method::GET,
            url!("/checkouts", id),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn checkout_reference_id(
        &self,
        reference_id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Checkout> {
        Self::send(
            reqwest::Method::GET,
            &format!("{}?checkout_reference={}", url!("/checkouts"), reference_id),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn checkout_delete(&self, id: &str, access_token: &crate::AccessToken) -> crate::Result {
        Self::send(
            reqwest::Method::DELETE,
            url!("/checkouts", id),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn checkout_update(
        &self,
        id: &str,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        Self::send(
            reqwest::Method::PUT,
            url!("/checkouts", id),
            Some(payload),
            Some(access_token),
        )
    }

    pub fn customer_create(
        &self,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        Self::send(
            reqwest::Method::POST,
            url!("/v0.1/customers"),
            Some(payload),
            Some(access_token),
        )
    }

    pub fn customer_update(
        &self,
        id: &str,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Customer> {
        Self::send(
            reqwest::Method::PUT,
            url!("/v0.1/customers", id),
            Some(payload),
            Some(access_token),
        )
    }

    pub fn customer_get(
        &self,
        id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Customer> {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/customers", id),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn customer_payment_instruments(
        &self,
        customer_id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Card>> {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/customers", customer_id, "payment-instruments"),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn customer_create_payment_instruments(
        &self,
        customer_id: &str,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Card> {
        Self::send(
            reqwest::Method::POST,
            url!("/v0.1/customers", customer_id, "payment-instruments"),
            Some(payload),
            Some(access_token),
        )
    }

    pub fn customer_delete_payment_instruments(
        &self,
        customer_id: &str,
        card_token: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        Self::send(
            reqwest::Method::DELETE,
            url!("/customers", customer_id, "payment-instruments", card_token),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn merchants_payment_methods(
        &self,
        merchant_code: &str,
        amount: Option<f32>,
        currency: Option<&str>,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::PaymentMethod>> {
        let mut url = url!("/v0.1/merchants", merchant_code, "payment-methods?").to_string();

        if let Some(amount) = amount {
            url.push_str(&format!("amount={amount}&"));
        }

        if let Some(currency) = currency {
            url.push_str(&format!("currency={currency}"));
        }

        Self::send(reqwest::Method::GET, &url, None::<()>, Some(access_token))
    }

    pub fn personal_get(
        &self,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::PersonalProfile> {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/me/personal-profile"),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn profile_get(&self, access_token: &crate::AccessToken) -> crate::Result<crate::Profile> {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/me/merchant-profile"),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn profile_update(
        &self,
        profile: &crate::Profile,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        Self::send(
            reqwest::Method::PUT,
            url!("/v0.1/me/merchant-profile"),
            Some(profile),
            Some(access_token),
        )
    }

    pub fn profile_doing_business_as_get(
        &self,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::DoingBusinessAs> {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/me/merchant-profile/doing-business-as"),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn profile_doing_business_as_update(
        &self,
        dba: &crate::DoingBusinessAs,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::DoingBusinessAs> {
        Self::send(
            reqwest::Method::PUT,
            url!("/v0.1/me/merchant-profile/doing-business-as"),
            Some(dba),
            Some(access_token),
        )
    }

    pub fn profile_bank_accounts(
        &self,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::BankAccount>> {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/me/merchant-profile/bank-accounts"),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn profile_settings(
        &self,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Settings> {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/me/merchant-profile/settings"),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn payouts_list(
        &self,
        filter: &crate::services::payouts::Filter,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Payout>> {
        Self::send(
            reqwest::Method::GET,
            &format!(
                "{}?{}",
                url!("/v0.1/me/financials/payouts"),
                filter.to_string()
            ),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn subaccounts_create(
        &self,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::SubAccount> {
        Self::send(
            reqwest::Method::POST,
            url!("/v0.1/me/accounts"),
            Some(payload),
            Some(access_token),
        )
    }

    pub fn subaccounts_delete(
        &self,
        id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::SubAccount> {
        Self::send(
            reqwest::Method::DELETE,
            url!("/v0.1/me/accounts", id),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn subaccounts_list(
        &self,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::SubAccount>> {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/me/accounts"),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn subaccounts_update(
        &self,
        id: &str,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::SubAccount> {
        Self::send(
            reqwest::Method::PUT,
            url!("/v0.1/me/accounts", id),
            Some(payload),
            Some(access_token),
        )
    }

    pub fn transactions_list(
        &self,
        filter: &crate::services::payouts::Filter,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Transaction>> {
        Self::send(
            reqwest::Method::GET,
            &format!(
                "{}?{}",
                url!("/v0.1/me/financials/transactions"),
                filter.to_string()
            ),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn transactions_get(
        &self,
        id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Transaction> {
        Self::send(
            reqwest::Method::GET,
            &format!("{}?id={id}", url!("/v0.1/me/transactions")),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn transactions_get_by_internal_id(
        &self,
        internal_id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Transaction> {
        Self::send(
            reqwest::Method::GET,
            &format!(
                "{}?internal_id={internal_id}",
                url!("/v0.1/me/transactions")
            ),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn transactions_get_by_code(
        &self,
        transaction_code: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Transaction> {
        Self::send(
            reqwest::Method::GET,
            &format!(
                "{}?transaction_code={transaction_code}",
                url!("/v0.1/me/transactions")
            ),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn transactions_history(
        &self,
        filter: &crate::services::transactions::Filter,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Transaction>> {
        Self::send(
            reqwest::Method::GET,
            &format!(
                "{}?{}",
                url!("/v0.1/me/financials/payouts"),
                filter.to_string()
            ),
            None::<()>,
            Some(access_token),
        )
    }

    pub fn transactions_refund(
        &self,
        id: u32,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        Self::send(
            reqwest::Method::GET,
            url!("/v0.1/me/refund", id),
            Some(payload),
            Some(access_token),
        )
    }

    pub fn transactions_get_receipt(
        &self,
        id: u32,
        merchant_id: u32,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Receipt> {
        Self::send(
            reqwest::Method::GET,
            &format!("{}?mid={merchant_id}", url!("/receipts", id)),
            None::<()>,
            Some(access_token),
        )
    }

    fn send<T: serde::de::DeserializeOwned>(
        method: reqwest::Method,
        url: &str,
        payload: Option<impl serde::Serialize>,
        access_token: Option<&crate::AccessToken>,
    ) -> crate::Result<T> {
        log::trace!("-> {method:?} {url}");

        let mut request = reqwest::blocking::Client::new().request(method, url);

        if let Some(access_token) = access_token {
            let bearer = access_token.bearer();
            log::trace!("-> Authorization: {bearer}");
            request = request.header(reqwest::header::AUTHORIZATION, &bearer);
        }

        if let Some(payload) = payload {
            if log::log_enabled!(log::Level::Trace) {
                log::trace!("-> {}", serde_json::to_string(&payload)?);
            }
            request = request.json(&payload);
        }

        let response = request.send()?;
        let status = response.status();
        let content = response.text()?;
        log::trace!("<- {content}");

        if status.is_success() {
            serde_json::from_str(&content).map_err(crate::Error::from)
        } else {
            let message = serde_json::from_str(&content)?;
            Err(crate::Error::Api(message))
        }
    }
}
