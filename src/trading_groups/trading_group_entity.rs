service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[my_no_sql_entity("trading-groups")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradingGroupMyNoSqlEntity {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Assets")]
    pub assets: Option<Vec<String>>,

    #[serde(rename = "TradingConditionsProfileId")]
    pub trading_conditions_profile_id: String,

    pub default: bool,
}

impl TradingGroupMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "tg";

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
