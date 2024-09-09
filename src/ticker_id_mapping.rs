service_sdk::macros::use_my_no_sql_entity!();

use serde::{Deserialize, Serialize};

#[my_no_sql_entity("ticker-id-mapping")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickerIdMappingMyNoSqlEntity {
    pub ids: Vec<String>,
}

impl TickerIdMappingMyNoSqlEntity {
    pub fn generate_partition_key(src_id: &str) -> &str {
        src_id
    }

    pub fn generate_row_key(src_ticker_id: &str) -> &str {
        src_ticker_id
    }

    pub fn get_src_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_ticker_id(&self) -> &str {
        &self.row_key
    }
}
