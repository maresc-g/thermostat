use chrono::{DateTime, FixedOffset};
use warp::{http, Filter};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use super::db;

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
        .and(with_db(dbitf))
        .and_then(get_temperature_history);

    warp::serve(temperature_history)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn get_temperature_history(thr: TemperatureHistoryRequest, db: Db) -> Result<impl warp::Reply, Infallible> {
    let res = db.lock().await.query("temperature/select_by_min_max", &[&thr.min_date, &thr.max_date]).await;
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&db::temperature::to_temperature_history_vec(res.unwrap())).unwrap()),
        http::StatusCode::OK,
    ))
}
