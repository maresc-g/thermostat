use warp::{Filter, http};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use super::db;
use crate::structs::TemperatureHistoryRequest;

type Db = Arc<Mutex<db::DbItf>>;

pub fn create_routes(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    get_current_temperature_route(db)
        .or(get_temperature_history_route(db))
}

fn get_current_temperature_route(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("v1" / "temperatures")
        .and(warp::get())
        .and(super::with_db(db.clone()))
        .and_then(get_current_temperature)
}

fn get_temperature_history_route(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("v1" / "temperatures")
        .and(warp::get())
        .and(warp::query::<TemperatureHistoryRequest>())
        .and(super::with_db(db.clone()))
        .and_then(get_temperature_history)
}

async fn get_current_temperature(db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut dbitf = db.lock().await;
    let t = dbitf.transaction().await;
    let res = db::setting::get_float_by_key(&t, &"current_temperature").await;
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&res).unwrap()),
        http::StatusCode::OK,
    ))
}

async fn get_temperature_history(thr: TemperatureHistoryRequest, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut dbitf = db.lock().await;
    let t = dbitf.transaction().await;
    let res = db::temperature::get_history(&t, &thr).await;
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&res).unwrap()),
        http::StatusCode::OK,
    ))
}