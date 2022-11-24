#[derive(Clone, Debug)]
pub struct Merchant<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Merchant<'a> {
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    pub fn profile(&self) -> crate::Result<crate::Profile> {
        self.api.profile_get(self.access_token)
    }

    pub fn update_profile(&self, profile: &crate::Profile) -> crate::Result {
        self.api.profile_update(profile, self.access_token)
    }

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

    pub fn bank_accounts(&self) -> crate::Result<Vec<crate::BankAccount>> {
        self.api.profile_bank_accounts(self.access_token)
    }

    pub fn settings(&self) -> crate::Result<crate::Settings> {
        self.api.profile_settings(self.access_token)
    }
}
