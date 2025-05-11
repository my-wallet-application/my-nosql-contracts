service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

pub const DEFAULT_GROUP_ID: &'static str = "default";

#[my_no_sql_entity("mt-trading-groups")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MtTradingGroup {
    pub name: String,
    pub leverages: Vec<i64>,
}

impl MtTradingGroup {
    pub const PARTITION_KEY: &'static str = "g";

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
