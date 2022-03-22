use chrono::{DateTime, FixedOffset};
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::row::Row;
use crate::structs::TemperatureHistoryRequest;

#[derive(Deserialize, Serialize, Debug)]
pub struct TemperatureHistory {
    temperature: f64,
    date: DateTime<FixedOffset>
}

pub(super) async fn prepare_all(db: &mut super::DbItf) {
    db.prepare_from_file("temperature/select_by_min_max").await;
    db.prepare_from_file("temperature/insert").await;
}

pub async fn insert(db: &super::DbItf, temperature: f64) {
    db.query("temperature/insert", &[&temperature]).await.unwrap();
}

pub async fn get_history(db: &super::DbItf, thr: &TemperatureHistoryRequest) -> Vec<TemperatureHistory> {
    let res = db.query("temperature/select_by_min_max", &[&thr.min_date, &thr.max_date]).await.unwrap();
    to_temperature_history_vec(res)
}

pub fn to_temperature_history(row: Row) -> TemperatureHistory {
    TemperatureHistory {
        temperature: row.get("temperature"),
        date: row.get("date")
    }
}

pub fn to_temperature_history_vec(rows: Vec<Row>) -> Vec<TemperatureHistory> {
    let mut res = Vec::new();
    for r in rows {
        res.push(to_temperature_history(r));
    }
    res
}