use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use std::ops::Deref;
use warp::{Filter, http};
use super::db;
use crate::structs::{DeleteHeaterTimeSlot, HeaterTimeSlot};

type Db = Arc<Mutex<db::DbItf>>;

pub fn create_routes(db: &Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_timeslot_route(db)
        .or(add_timeslot_route(db))
        .or(update_timeslot_route(db))
        .or(delete_timeslot_route(db))
}

fn get_timeslot_route(db: &Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "heater_timeslot")
        .and(warp::get())
        .and(super::with_db(db.clone()))
        .and_then(get_timeslot)
}

fn add_timeslot_route(db: &Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "heater_timeslot")
        .and(warp::post())
        .and(heater_timeslot_json_body())
        .and(super::with_db(db.clone()))
        .and_then(add_timeslot)
}

fn update_timeslot_route(db: &Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "heater_timeslot")
        .and(warp::put())
        .and(heater_timeslot_json_body())
        .and(super::with_db(db.clone()))
        .and_then(update_timeslot)
}

fn delete_timeslot_route(db: &Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "heater_timeslot")
        .and(warp::delete())
        .and(delete_heater_timeslot_json_body())
        .and(super::with_db(db.clone()))
        .and_then(delete_timeslot)
}

async fn get_timeslot(db: Db) -> Result<impl warp::Reply, Infallible> {
    let res = db::heater_timeslot::get(&db.lock().await.deref()).await;
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&res).unwrap()),
        http::StatusCode::OK,
    ))
}

async fn add_timeslot(ts: HeaterTimeSlot, db: Db) -> Result<impl warp::Reply, Infallible> {
    db::heater_timeslot::insert(&db.lock().await.deref(), &ts).await;
    Ok(warp::reply::with_status(
        "Ok",
        http::StatusCode::OK,
    ))
}

async fn update_timeslot(ts: HeaterTimeSlot, db: Db) -> Result<impl warp::Reply, Infallible> {
    db::heater_timeslot::update(&db.lock().await.deref(), &ts).await;
    Ok(warp::reply::with_status(
        "Ok",
        http::StatusCode::OK,
    ))
}

async fn delete_timeslot(dhts: DeleteHeaterTimeSlot, db: Db) -> Result<impl warp::Reply, Infallible> {
    db::heater_timeslot::delete(&db.lock().await.deref(), &dhts.pk).await;
    Ok(warp::reply::with_status(
        "Ok",
        http::StatusCode::OK,
    ))
}

fn heater_timeslot_json_body() -> impl Filter<Extract = (HeaterTimeSlot,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn delete_heater_timeslot_json_body() -> impl Filter<Extract = (DeleteHeaterTimeSlot,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024).and(warp::body::json())
}
