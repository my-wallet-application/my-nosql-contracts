use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("bid-ask-snapshot")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAskSnapshotNoSqlEntity {
    pub moment: i64,
    pub bid: f64,
    pub ask: f64,
}

impl BidAskSnapshotNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "s"
    }

    pub fn generate_row_key(instrument_id: &'static str) -> &'static str {
        instrument_id
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.row_key
    }
}
