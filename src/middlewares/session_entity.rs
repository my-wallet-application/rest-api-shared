use serde::{Deserialize, Serialize};
use service_sdk::{my_no_sql_sdk, rust_extensions::date_time::DateTimeAsMicroseconds};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionClaim {
    pub name: String,
    pub expires: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

#[service_sdk::my_no_sql_sdk::macros::my_no_sql_entity("sessionsentites")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntity {
    #[serde(rename = "TraderId")]
    pub trader_id: String,
    #[serde(rename = "Expires")]
    pub expires: String,
    #[serde(rename = "Claims")]
    pub claims: Option<Vec<SessionClaim>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl SessionEntity {
    pub const PARTITION_KEY: &'static str = "t";

    pub fn get_session_token(&self) -> &str {
        &self.row_key
    }

    pub fn extend_expiration(&mut self, new_expiration: DateTimeAsMicroseconds) {
        self.expires = new_expiration.to_rfc3339();
    }

    pub fn set_claim(&mut self, name: String, expires: DateTimeAsMicroseconds, ip: Option<String>) {
        if self.claims.is_none() {
            self.claims = Some(vec![SessionClaim {
                name,
                expires: expires.unix_microseconds,
                ip,
            }]);
            return;
        }

        let claims = self.claims.as_mut().unwrap();

        claims.retain(|claim| claim.name != name);

        claims.push(SessionClaim {
            name,
            expires: expires.unix_microseconds,
            ip,
        });
    }
}
