#[derive(Clone, Debug)]
pub struct Checkout<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Checkout<'a> {
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    pub fn payment_methods(&self) -> crate::Result<crate::PaymentMethod> {
        todo!()
    }

    pub fn create(&self, checkout: &crate::Checkout) -> crate::Result {
        self.api.checkout_create(checkout, self.access_token)
    }

    pub fn list(&self) -> crate::Result<Vec<crate::Checkout>> {
        todo!()
    }

    pub fn find_by_id(&self, id: &str) -> crate::Result<crate::Checkout> {
        self.api.checkout_get(id, self.access_token)
    }

    pub fn find_by_reference_id(&self, reference_id: &str) -> crate::Result<crate::Checkout> {
        self.api
            .checkout_reference_id(reference_id, self.access_token)
    }

    pub fn delete(&self, id: &str) -> crate::Result {
        self.api.checkout_delete(id, self.access_token)
    }

    pub fn pay(
        &self,
        id: &str,
        customer_id: &str,
        card_token: &str,
        installments: Option<u8>,
    ) -> crate::Result {
        let payload = ureq::json!({
            "payment_type": "card",
            "customer_id": customer_id,
            "token": card_token,
            "installments": installments.unwrap_or(1),
        });

        self.api.checkout_update(id, &payload, self.access_token)
    }
}
