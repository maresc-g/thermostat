use chrono::{Date, DateTime, Utc};
use warp::{http, Filter};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use crate::db::DbItf;
use super::db;

#[derive(Deserialize, Serialize, Debug)]
struct TemperatureHistoryRequest {
    min_date: DateTime<Utc>,
    max_date: DateTime<Utc>
}

type Db = Arc<Mutex<db::DbItf>>;

pub async fn run_http_server() {
    let dbitf = Arc::new(Mutex::new(db::DbItf::new()));
    // let dbitf_filter = warp::any().map(move || dbitf.clone());
    let temperature_history = warp::path!("v1" / "temperatures")
        .and(with_db(dbitf.clone()))
        .and(warp::get())
        .and_then(test);


    // let get = warp::get()
    //     .and(temperature_history);
    //
    // let post = warp::post();
    //
    // let routes = get;
    //     // .or(post);

    warp::serve(temperature_history)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub async fn test(db: Db) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}
