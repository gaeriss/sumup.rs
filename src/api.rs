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
        ureq::post(url!("/token"))
            .send_json(payload)?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn account_get(&self, access_token: &crate::AccessToken) -> crate::Result<crate::Account> {
        ureq::get(url!("/v0.1/me"))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn checkout_create(
        &self,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        ureq::post(url!("/checkouts"))
            .set("Authorization", &access_token.bearer())
            .send_json(payload)?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn checkout_list(
        &self,
        checkout_reference: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Checkout>> {
        ureq::get(&format!("{}?checkout_reference={checkout_reference}", url!("/v0.1/checkouts")))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn checkout_get(
        &self,
        id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Checkout> {
        ureq::get(url!("/checkouts", id))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn checkout_reference_id(
        &self,
        reference_id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Checkout> {
        ureq::get(&format!(
            "{}?checkout_reference={}",
            url!("/checkouts"),
            reference_id
        ))
        .set("Authorization", &access_token.bearer())
        .call()?
        .into_json()
        .map_err(crate::Error::from)
    }

    pub fn checkout_delete(&self, id: &str, access_token: &crate::AccessToken) -> crate::Result {
        ureq::delete(url!("/checkouts", id))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)?;

        Ok(())
    }

    pub fn checkout_update(
        &self,
        id: &str,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        ureq::put(url!("/checkouts", id))
            .set("Authorization", &access_token.bearer())
            .send_json(payload)?
            .into_json()
            .map_err(crate::Error::from)?;

        Ok(())
    }

    pub fn customer_create(
        &self,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        ureq::post(url!("/v0.1/customers"))
            .set("Authorization", &access_token.bearer())
            .send_json(payload)
            .map(|_| ())
            .map_err(crate::Error::from)
    }

    pub fn customer_update(
        &self,
        id: &str,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Customer> {
        ureq::put(url!("/v0.1/customers", id))
            .set("Authorization", &access_token.bearer())
            .send_json(payload)?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn customer_get(
        &self,
        id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Customer> {
        ureq::get(url!("/v0.1/customers", id))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn customer_payment_instruments(
        &self,
        customer_id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Card>> {
        ureq::get(url!("/v0.1/customers", customer_id, "payment-instruments"))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn customer_create_payment_instruments(
        &self,
        customer_id: &str,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Card> {
        ureq::post(url!("/v0.1/customers", customer_id, "payment-instruments"))
            .set("Authorization", &access_token.bearer())
            .send_json(payload)?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn customer_delete_payment_instruments(
        &self,
        customer_id: &str,
        card_token: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        ureq::delete(url!(
            "/customers",
            customer_id,
            "payment-instruments",
            card_token
        ))
        .set("Authorization", &access_token.bearer())
        .call()?
        .into_json()
        .map_err(crate::Error::from)
    }

    pub fn personal_get(
        &self,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::PersonalProfile> {
        ureq::get(url!("/v0.1/me/personal-profile"))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn profile_get(&self, access_token: &crate::AccessToken) -> crate::Result<crate::Profile> {
        ureq::get(url!("/v0.1/me/merchant-profile"))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn profile_update(
        &self,
        profile: &crate::Profile,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        ureq::put(url!("/v0.1/me/merchant-profile"))
            .set("Authorization", &access_token.bearer())
            .send_json(profile)?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn profile_doing_business_as_get(
        &self,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::DoingBusinessAs> {
        ureq::get(url!("/v0.1/me/merchant-profile/doing-business-as"))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn profile_doing_business_as_update(
        &self,
        dba: &crate::DoingBusinessAs,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::DoingBusinessAs> {
        ureq::put(url!("/v0.1/me/merchant-profile/doing-business-as"))
            .set("Authorization", &access_token.bearer())
            .send_json(dba)?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn profile_bank_accounts(
        &self,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::BankAccount>> {
        ureq::get(url!("/v0.1/me/merchant-profile/bank-accounts"))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn profile_settings(
        &self,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Settings> {
        ureq::get(url!("/v0.1/me/merchant-profile/settings"))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn payouts_list(
        &self,
        filter: &crate::services::payouts::Filter,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Payout>> {
        ureq::get(&format!(
            "{}?{}",
            url!("/v0.1/me/financials/payouts"),
            filter.to_string()
        ))
        .set("Authorization", &access_token.bearer())
        .call()?
        .into_json()
        .map_err(crate::Error::from)
    }

    pub fn transactions_list(
        &self,
        filter: &crate::services::payouts::Filter,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Transaction>> {
        ureq::get(&format!(
            "{}?{}",
            url!("/v0.1/me/financials/transactions"),
            filter.to_string()
        ))
        .set("Authorization", &access_token.bearer())
        .call()?
        .into_json()
        .map_err(crate::Error::from)
    }

    pub fn transactions_get(
        &self,
        id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Transaction> {
        ureq::get(&format!("{}?id={id}", url!("/v0.1/me/transactions")))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn transactions_get_by_internal_id(
        &self,
        internal_id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Transaction> {
        ureq::get(&format!(
            "{}?internal_id={internal_id}",
            url!("/v0.1/me/transactions")
        ))
        .set("Authorization", &access_token.bearer())
        .call()?
        .into_json()
        .map_err(crate::Error::from)
    }

    pub fn transactions_get_by_code(
        &self,
        transaction_code: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Transaction> {
        ureq::get(&format!(
            "{}?transaction_code={transaction_code}",
            url!("/v0.1/me/transactions")
        ))
        .set("Authorization", &access_token.bearer())
        .call()?
        .into_json()
        .map_err(crate::Error::from)
    }

    pub fn transactions_history(
        &self,
        filter: &crate::services::transactions::Filter,
        access_token: &crate::AccessToken,
    ) -> crate::Result<Vec<crate::Transaction>> {
        ureq::get(&format!(
            "{}?{}",
            url!("/v0.1/me/financials/payouts"),
            filter.to_string()
        ))
        .set("Authorization", &access_token.bearer())
        .call()?
        .into_json()
        .map_err(crate::Error::from)
    }

    pub fn transactions_refund(
        &self,
        id: u32,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        ureq::get(url!("/v0.1/me/refund", id))
            .set("Authorization", &access_token.bearer())
            .send_json(payload)?
            .into_json()
            .map_err(crate::Error::from)
    }

    pub fn transactions_get_receipt(
        &self,
        id: u32,
        merchant_id: u32,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Receipt> {
        ureq::get(&format!("{}?mid={merchant_id}", url!("/receipts", id)))
            .set("Authorization", &access_token.bearer())
            .call()?
            .into_json()
            .map_err(crate::Error::from)
    }
}
