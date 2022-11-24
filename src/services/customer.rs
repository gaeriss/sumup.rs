#[derive(Clone, Debug)]
pub struct Customer<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Customer<'a> {
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    pub fn create(&self, id: u32, details: &crate::Details) -> crate::Result<crate::Customer> {
        let payload = ureq::json!({
            "customer_id": id,
            "personal_details": details,
        });

        self.api.customer_create(&payload, self.access_token)
    }

    pub fn update(&self, id: u32, details: &crate::Details) -> crate::Result {
        let payload = ureq::json!({
            "customer_id": id,
            "personal_details": details,
        });

        self.api.customer_update(id, &payload, self.access_token)
    }

    pub fn get(&self, id: u32) -> crate::Result<crate::Customer> {
        self.api.customer_get(id, self.access_token)
    }

    pub fn payment_instruments(&self, id: u32) -> crate::Result<Vec<String>> {
        self.api.customer_payment_instruments(id, self.access_token)
    }

    pub fn delete_payment_instruments(&self, id: u32, card_token: &str) -> crate::Result {
        self.api
            .customer_delete_payment_instruments(id, card_token, self.access_token)
    }
}
