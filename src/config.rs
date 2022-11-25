#[derive(Clone, Debug, Default)]
pub struct Config {
    /**
     * This is the client id that you receive after you
     * [register](https://developer.sumup.com/docs/register-app) your application in SumUp
     */
    pub client_id: String,
    /** This is the client secret that corresponds to the client id */
    pub client_secret: String,
    /** This indicates which authorization flow should be used to acquire OAuth token */
    pub grant_type: GrantType,
    /**
     * This is an array with all the [authorization
     * scopes](https://developer.sumup.com/docs/authorization#authorization-scopes) that you need
     * for your application
     */
    pub scopes: Scopes,
    /**
     * This is the code returned at the last step from [authorization code
     * flow](https://developer.sumup.com/docs/authorization#authorization-flows)
     */
    pub code: Option<String>,
    /** This is your SumUp's username if you want to use password authorization flow */
    pub username: Option<String>,
    /** This is your SumUp's password if you want to use password authorization flow */
    pub password: Option<String>,
    /**
     * This is the value of a valid access token that is acquired through other methods. It is used
     * if you don't want to request new access token
     */
    pub access_token: Option<String>,
    /** This is the refresh token through which can be requested new access token */
    pub refresh_token: Option<String>,
}

impl Config {
    pub fn new(client_id: &str, client_secret: &str, code: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            code: Some(code.to_string()),
            scopes: vec![Scope::TransactionsHistory].into(),

            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub enum GrantType {
    #[default]
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "client_credentials")]
    ClientCredentials,
    #[serde(rename = "password")]
    Password,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scope {
    Payments,
    TransactionsHistory,
    UserAppSettings,
    UserProfileReadonly,
    UserProfile,
    UserSubaccounts,
    UserPayoutSettings,
    Balance,
    Products,
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Payments => "payments",
            Self::TransactionsHistory => "transactions.history",
            Self::UserAppSettings => "user.app-settings",
            Self::UserProfileReadonly => "user.profile_readonly",
            Self::UserProfile => "user.profile",
            Self::UserSubaccounts => "user.subaccounts",
            Self::UserPayoutSettings => "user.payout-settings",
            Self::Balance => "balance",
            Self::Products => "products",
        };

        f.write_str(s)
    }
}

impl std::str::FromStr for Scope {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scope = match s {
            "payments" => Self::Payments,
            "transactions.history" => Self::TransactionsHistory,
            "user.app-settings" => Self::UserAppSettings,
            "user.profile_readonly" => Self::UserProfileReadonly,
            "user.profile" => Self::UserProfile,
            "user.subaccounts" => Self::UserSubaccounts,
            "user.payout-settings" => Self::UserPayoutSettings,
            "balance" => Self::Balance,
            "products" => Self::Products,
            _ => return Err(crate::Error::InvalidScope(s.to_string())),
        };

        Ok(scope)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Scopes(Vec<Scope>);

impl serde::Serialize for Scopes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl std::fmt::Display for Scopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();

        if let Some(first) = iter.next() {
            write!(f, "{first}")?;
        }

        for scope in iter {
            write!(f, " {scope}")?;
        }

        Ok(())
    }
}

impl<'de> serde::Deserialize<'de> for Scopes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(Visitor::new()).map(Self)
    }
}

struct Visitor;

impl Visitor {
    fn new() -> Self {
        Self
    }
}

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Vec<Scope>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Scope should be a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value.is_empty() {
            return Ok(Vec::new());
        }

        value
            .split(' ')
            .map(|x| x.parse())
            .collect::<crate::Result<Vec<_>>>()
            .map_err(serde::de::Error::custom)
    }
}

impl std::ops::Deref for Scopes {
    type Target = Vec<Scope>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Scope>> for Scopes {
    fn from(scopes: Vec<Scope>) -> Self {
        Self(scopes)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn serialize_scopes() -> crate::Result {
        let scopes = crate::config::Scopes::from(vec![
            crate::config::Scope::Payments,
            crate::config::Scope::Products,
        ]);

        assert_eq!(serde_json::to_string(&scopes)?, "\"payments products\"");

        Ok(())
    }

    #[test]
    fn deserialize_scopes() -> crate::Result {
        let scopes = crate::config::Scopes::from(vec![
            crate::config::Scope::Payments,
            crate::config::Scope::Products,
        ]);

        assert_eq!(scopes, serde_json::from_str("\"payments products\"")?);
        assert_eq!(
            crate::config::Scopes::default(),
            serde_json::from_str("\"\"")?
        );

        Ok(())
    }
}
