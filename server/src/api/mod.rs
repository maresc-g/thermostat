mod heater_timeslot;

use chrono::{DateTime, FixedOffset};
use warp::{http, Filter};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use super::db;
use crate::structs::HeaterTimeSlot::HeaterTimeSlot;

#[derive(Deserialize, Serialize, Debug)]
struct TemperatureHistoryRequest {
    min_date: DateTime<FixedOffset>,
    max_date: DateTime<FixedOffset>
}

type Db = Arc<Mutex<db::DbItf>>;

pub async fn run_http_server() {
    let dbitf = Arc::new(Mutex::new(db::DbItf::new().await));
    let temperature_history = warp::path!("v1" / "temperatures")
        .and(warp::get())
        .and(warp::query::<TemperatureHistoryRequest>())
        .and(with_db(dbitf.clone()))
        .and_then(get_temperature_history);

    let add_timeslot = warp::path!("v1" / "heater_timeslot")
        .and(warp::post())
        .and(json_body())
        .and(with_db(dbitf.clone()))
        .and_then(heater_timeslot::add_timeslot);

    let routes = temperature_history.or(add_timeslot);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (HeaterTimeSlot,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn get_temperature_history(thr: TemperatureHistoryRequest, db: Db) -> Result<impl warp::Reply, Infallible> {
    let res = db.lock().await.query("temperature/select_by_min_max", &[&thr.min_date, &thr.max_date]).await;
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&db::temperature::to_temperature_history_vec(res.unwrap())).unwrap()),
        http::StatusCode::OK,
    ))
}
