#[derive(Clone, Debug, Default)]
pub struct Api {}

macro_rules! url {
    ($path:literal) => {
        concat!("https://api.sumup.com/v1.0", $path)
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
}
