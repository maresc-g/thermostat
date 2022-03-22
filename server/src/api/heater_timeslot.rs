use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use warp::http;
use super::db;
use crate::structs::HeaterTimeSlot::HeaterTimeSlot;

type Db = Arc<Mutex<db::DbItf>>;

pub(super) async fn add_timeslot(ts: HeaterTimeSlot, db: Db) -> Result<impl warp::Reply, Infallible> {
    db.lock().await.query("heater_timeslot/insert", &[&ts.target_temperature, &ts.start_day, &ts.start_time, &ts.end_day, &ts.end_time]).await.unwrap();
    Ok(warp::reply::with_status(
        "Ok",
        http::StatusCode::OK,
    ))
}