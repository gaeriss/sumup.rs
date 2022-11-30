#[derive(Clone, Debug)]
pub struct Subaccounts<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Subaccounts<'a> {
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    /**
     * <https://developer.sumup.com/docs/api/create-a-subaccount/>
     */
    pub fn create(&self, email: &str, password: &str) -> crate::Result<crate::SubAccount> {
        let payload = ureq::json!({
            "username": email,
            "password": password,
        });

        self.api.subaccounts_create(&payload, self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/list-subaccounts/>
     */
    pub fn list(&self) -> crate::Result<Vec<crate::SubAccount>> {
        self.api.subaccounts_list(self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/update-a-subaccount/>
     */
    pub fn update(
        &self,
        id: &str,
        new_username: Option<&str>,
        new_password: Option<&str>,
    ) -> crate::Result<crate::SubAccount> {
        let mut payload = ureq::json!({});

        if let Some(new_username) = new_username {
            payload["username"] = ureq::json!(new_username);
        }

        if let Some(new_password) = new_password {
            payload["password"] = ureq::json!(new_password);
        }

        self.api.subaccounts_update(id, &payload, self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/deactivate-a-subaccount/>
     */
    pub fn delete(&self, id: &str) -> crate::Result<crate::SubAccount> {
        self.api.subaccounts_delete(id, self.access_token)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn create() -> crate::Result {
        let api = crate::test::api()?;
        let subaccounts = api.subaccounts();

        subaccounts.create("sb@example.org", "pass1")?;

        let accounts = subaccounts.list()?;

        if accounts.is_empty() {
            log::warn!("Empty response");
        } else {
            assert_eq!(
                subaccounts
                    .update(&accounts[0].id, Some("sb_new@example.org"), None)?
                    .username,
                "sb_new@example.org"
            );
            assert!(subaccounts.delete(&accounts[0].id)?.disabled);
        }

        Ok(())
    }
}
