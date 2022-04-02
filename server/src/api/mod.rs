mod heater_timeslot;
mod temperature;
mod setting;

use warp::{Filter};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::Method;
use super::db;

type Db = Arc<Mutex<db::DbItf>>;

pub async fn run_http_server() {
    let db = Arc::new(Mutex::new(db::DbItf::new().await));
    let cors = warp::cors()
        .allow_origin("http://127.0.0.1:3000")
        .allow_origin("http://localhost:3000")
        .allow_header("content-type")
        .allow_methods([Method::GET, Method::POST, Method::PUT]);

    let routes = heater_timeslot::create_routes(&db)
        .or(temperature::create_routes(&db))
        .or(setting::create_routes(&db))
        .with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
