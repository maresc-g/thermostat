use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use std::ops::Deref;
use warp::{Filter, http};
use super::db;
use crate::structs::HeaterTimeSlot;

type Db = Arc<Mutex<db::DbItf>>;

pub fn create_routes(db: &Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    add_timeslot_route(db)
}

fn add_timeslot_route(db: &Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "heater_timeslot")
        .and(warp::post())
        .and(heater_timeslot_json_body())
        .and(super::with_db(db.clone()))
        .and_then(add_timeslot)
}

pub(super) async fn add_timeslot(ts: HeaterTimeSlot, db: Db) -> Result<impl warp::Reply, Infallible> {
    db::heater_timeslot::insert(&db.lock().await.deref(), &ts).await;
    Ok(warp::reply::with_status(
        "Ok",
        http::StatusCode::OK,
    ))
}

fn heater_timeslot_json_body() -> impl Filter<Extract = (HeaterTimeSlot,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
