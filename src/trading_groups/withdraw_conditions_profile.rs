service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[my_no_sql_entity("withdraw-condition-profiles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WithdrawConditionsProfileMyNoSqlEntity {
    pub commission_percent: f64, // 100  means 100%
}

impl WithdrawConditionsProfileMyNoSqlEntity {
    pub fn get_profile_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_asset_id(&self) -> &str {
        &self.row_key
    }
}
