#[derive(Clone, Debug)]
pub struct Merchant<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Merchant<'a> {
    #[must_use]
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    pub fn profile(&self) -> crate::Result<crate::Profile> {
        self.api.profile_get(self.access_token)
    }

    pub fn update_profile(&self, profile: &crate::Profile) -> crate::Result {
        self.api.profile_update(profile, self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/retrieve-dba/>
     */
    pub fn doing_business_as(&self) -> crate::Result<crate::DoingBusinessAs> {
        self.api.profile_doing_business_as_get(self.access_token)
    }

    pub fn update_doing_business_as(
        &self,
        dba: &crate::DoingBusinessAs,
    ) -> crate::Result<crate::DoingBusinessAs> {
        self.api
            .profile_doing_business_as_update(dba, self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/list-bank-accounts/>
     */
    pub fn bank_accounts(&self) -> crate::Result<Vec<crate::BankAccount>> {
        self.api.profile_bank_accounts(self.access_token)
    }

    /**
     * <https://developer.sumup.com/docs/api/list-settings/>
     */
    pub fn settings(&self) -> crate::Result<crate::Settings> {
        self.api.profile_settings(self.access_token)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn profile() -> crate::Result {
        let api = crate::test::api()?;

        api.merchant().profile()?;

        Ok(())
    }

    #[test]
    fn update_profile() -> crate::Result {
        let api = crate::test::api()?;

        let mut profile = api.merchant().profile()?;
        profile.default_currency = "USD".to_string();

        api.merchant().update_profile(&profile)?;

        Ok(())
    }

    #[test]
    fn doing_business_as() -> crate::Result {
        let api = crate::test::api()?;

        let mut dba = api.merchant().doing_business_as()?;
        dba.dynamic_descriptor = "Update".to_string();

        api.merchant().update_doing_business_as(&dba)?;

        Ok(())
    }

    #[test]
    fn bank_accounts() -> crate::Result {
        let api = crate::test::api()?;

        api.merchant().bank_accounts()?;

        Ok(())
    }

    #[test]
    fn settings() -> crate::Result {
        let api = crate::test::api()?;

        api.merchant().settings()?;

        Ok(())
    }
}
