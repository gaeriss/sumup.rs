#[derive(Clone, Debug)]
pub struct Api {}

impl Api {
    pub fn new() -> Self {
        Self {}
    }

    pub fn token(
        &self,
        payload: impl serde::Serialize,
    ) -> crate::Result<crate::authorization::AccessToken> {
        let token = ureq::post("/token").send_json(payload)?.into_json()?;

        Ok(token)
    }
}
