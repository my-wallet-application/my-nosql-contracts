service_sdk::macros::use_my_no_sql_entity!();
use std::collections::HashMap;

use rust_extensions::StrOrString;
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("key-value")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyValueMyNoSqlEntity {
    pub key_value: HashMap<String, String>,
}

impl KeyValueMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "c"
    }

    pub fn generate_row_key<'s>(trader_id: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
        trader_id.into()
    }
}
