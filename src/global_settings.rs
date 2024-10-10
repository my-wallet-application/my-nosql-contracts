use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("global-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalSettingsMyNoSqlEntity {
    pub commissions_account_id: String,
}
