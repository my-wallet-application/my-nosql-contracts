use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("send-grid-settings")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SendGridSettingsMyNoSqlEntity {
    pub send_grid_api_key: String,
}

impl SendGridSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "sg"
    }

    pub fn generate_row_key(src_id: &'static str) -> &'static str {
        src_id
    }
}
