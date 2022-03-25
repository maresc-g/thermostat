use warp::{Filter, http};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use std::ops::Deref;
use super::db;
use crate::structs::TemperatureHistoryRequest;

type Db = Arc<Mutex<db::DbItf>>;

pub fn create_routes(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    get_temperature_history_route(db)
}

fn get_temperature_history_route(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("v1" / "temperatures")
        .and(warp::get())
        .and(warp::query::<TemperatureHistoryRequest>())
        .and(super::with_db(db.clone()))
        .and_then(get_temperature_history)
}

async fn get_temperature_history(thr: TemperatureHistoryRequest, db: Db) -> Result<impl warp::Reply, Infallible> {
    let res = db::temperature::get_history(db.lock().await.deref(), &thr).await;
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&res).unwrap()),
        http::StatusCode::OK,
    ))
}