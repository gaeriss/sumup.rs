#[derive(Clone, Debug)]
pub struct Customer<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Customer<'a> {
    #[must_use]
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    /**
     * <https://developer.sumup.com/docs/api/create-a-customer/>
     */
    pub fn create(&self, customer: &crate::Customer) -> crate::Result {
        self.api.customer_create(customer, self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/update-a-customer/>
     */
    pub fn update(&self, customer: &crate::Customer) -> crate::Result<crate::Customer> {
        self.api
            .customer_update(&customer.customer_id, customer, self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/retrieve-a-customer/>
     */
    pub fn get(&self, id: &str) -> crate::Result<crate::Customer> {
        self.api.customer_get(id, self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/list-payment-instruments/>
     */
    pub fn payment_instruments(&self, customer_id: &str) -> crate::Result<Vec<crate::Card>> {
        self.api
            .customer_payment_instruments(customer_id, self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/create-a-payment-instrument/>
     */
    pub fn create_payment_instruments(
        &self,
        customer_id: &str,
        card: &crate::Card,
    ) -> crate::Result<crate::Card> {
        let payload = ureq::json!({
            "type": "card",
            "card": card,
        });
        self.api
            .customer_create_payment_instruments(customer_id, payload, self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/deactivate-a-payment-instrument/>
     */
    pub fn delete_payment_instruments(&self, customer_id: &str, card_token: &str) -> crate::Result {
        self.api
            .customer_delete_payment_instruments(customer_id, card_token, self.access_token)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn customer() -> crate::Result {
        let api = crate::test::api()?;
        let api_customer = api.customer();

        let mut customer = crate::Customer {
            customer_id: "0".to_string(),
            personal_detail: crate::Details {
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
                email: Some("john.doe@example.org".to_string()),

                ..Default::default()
            },
        };

        api_customer.create(&customer)?;
        api_customer.get("0")?;

        customer.personal_detail.email = None;
        api_customer.update(&customer)?;

        let new_customer = api_customer.get("0")?;
        assert!(new_customer.personal_detail.email.is_none());

        Ok(())
    }

    #[test]
    fn payment() -> crate::Result {
        let api = crate::test::api()?;
        let api_customer = api.customer();

        let card = crate::Card {
            name: "FIRSTNAME LASTNAME".to_string(),
            number: "1234567890123456".to_string(),
            expiry_year: "2023".to_string(),
            expiry_month: "01".to_string(),
            cvv: "123".to_string(),
            zip_code: "12345".to_string(),
        };

        api_customer.create_payment_instruments("63827d30b0ccda0004457bc5", &card)?;
        let cards = api_customer.payment_instruments("63827d30b0ccda0004457bc5")?;
        api_customer.delete_payment_instruments("63827d30b0ccda0004457bc5", &cards[0].name)?;

        Ok(())
    }
}
