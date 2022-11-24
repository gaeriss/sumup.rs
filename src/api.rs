#[derive(Clone, Debug)]
pub struct Api {}

impl Api {
    pub fn new() -> Self {
        Self {}
    }

    pub fn token(&self, payload: impl serde::Serialize) -> crate::Result<crate::AccessToken> {
        let token = ureq::post("/token").send_json(payload)?.into_json()?;

        Ok(token)
    }

    pub fn checkout_create(
        &self,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result {
        todo!()
    }

    pub fn checkout_get(
        &self,
        id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Checkout> {
        todo!()
    }

    pub fn checkout_reference_id(
        &self,
        refence_id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Checkout> {
        todo!()
    }

    pub fn checkout_delete(
        &self,
        id: &str,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Checkout> {
        todo!()
    }

    pub fn checkout_update(
        &self,
        id: &str,
        payload: impl serde::Serialize,
        access_token: &crate::AccessToken,
    ) -> crate::Result<crate::Checkout> {
        todo!()
    }
}
