use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("global-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalSettingsMyNoSqlEntity {
    pub corporate_account_id: String,
}

impl GlobalSettingsMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "gs";
    pub const ROW_KEY: &'static str = "gs";
}
