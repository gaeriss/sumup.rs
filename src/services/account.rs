#[derive(Clone, Debug)]
pub struct Account<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Account<'a> {
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    /**
     * <https://developer.sumup.com/docs/api/retrieve-an-account/>
     */
    pub fn get(&self) -> crate::Result<crate::Account> {
        self.api.account_get(self.access_token)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get() -> crate::Result {
        let api = crate::test::api()?;

        api.account().get()?;

        Ok(())
    }
}
