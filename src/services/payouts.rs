#[derive(Clone, Debug)]
pub struct Payouts<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Payouts<'a> {
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    pub fn payouts(&self, filters: &Filter) -> crate::Result<Vec<crate::Payout>> {
        self.api.payouts_list(filters, self.access_token)
    }

    pub fn transactions(&self, filters: &Filter) -> crate::Result<Vec<crate::Transaction>> {
        self.api.transactions_list(filters, self.access_token)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Filter {
    start_date: String,
    end_date: String,
    limit: Option<u32>,
    descending_order: bool,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        let order = if self.descending_order { "desc" } else { "asc" };

        format!(
            "start_date={}&end_date={}&limit={}&order={order}&format=json",
            self.start_date,
            self.end_date,
            self.limit.unwrap_or(10)
        )
    }
}
