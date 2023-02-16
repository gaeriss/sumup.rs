#[derive(Clone, Debug)]
pub struct Personal<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Personal<'a> {
    #[must_use]
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    /**
     * <https://developer.sumup.com/docs/api/retrieve-a-profile/>
     */
    pub fn get(&self) -> crate::Result<crate::PersonalProfile> {
        self.api.personal_get(self.access_token)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get() -> crate::Result {
        let api = crate::test::api()?;

        api.personal().get()?;

        Ok(())
    }
}
