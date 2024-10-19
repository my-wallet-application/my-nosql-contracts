use std::collections::BTreeMap;

use serde::*;
use service_sdk::rust_extensions::date_time::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("mini-graph")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiniGraphNoSqlEntity {
    //pub data: Vec<f64>,
    pub candles: Option<BTreeMap<i64, [f64; 4]>>, //hour_key, open, high, low, close
}

impl MiniGraphNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "m"
    }

    pub fn generate_row_key(instrument_id: &'static str) -> &'static str {
        instrument_id
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.row_key
    }

    pub fn new(instrument_id: String, timestamp: DateTimeAsMicroseconds, price: f64) -> Self {
        let hour_key: IntervalKey<HourKey> = timestamp.into();
        let mut candles = BTreeMap::new();
        candles.insert(hour_key.to_i64(), [price, price, price, price]);

        Self {
            partition_key: MiniGraphNoSqlEntity::generate_partition_key().to_string(),
            row_key: instrument_id,
            time_stamp: String::new(),
            //data: vec![price],
            candles: Some(candles),
        }
    }

    /*
       pub fn insert_value(&mut self, value: f64) {
           self.data.push(value);
           if self.data.len() > 240 {
               self.data.remove(0);
           }
       }
    */
    pub fn update_candle(&mut self, hour_key: IntervalKey<HourKey>, price: f64) {
        if self.candles.is_none() {
            self.candles = Some(BTreeMap::new());
        }

        let candles = self.candles.as_mut().unwrap();
        if let Some(value) = candles.get_mut(hour_key.as_i64_ref()) {
            if price > value[1] {
                value[1] = price;
            }

            if price < value[2] {
                value[2] = price;
            }

            value[3] = price;
        } else {
            candles.insert(hour_key.to_i64(), [price, price, price, price]);
        }

        if candles.len() > 72 {
            candles.pop_first();
        }
    }
}
