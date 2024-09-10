use serde::*;

service_sdk::macros::use_my_no_sql_entity!();
#[derive(Serialize, Deserialize, Clone)]
pub struct PriceBridgesSettings {
    pub host_port: String,
}

impl PriceBridgesSettings {
    pub fn generate_partition_key() -> &'static str {
        "pb"
    }

    pub fn generate_row_key(src_id: &'static str) -> &'static str {
        src_id
    }
}
