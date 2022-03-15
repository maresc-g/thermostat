use warp::Filter;

pub async fn run_http_server() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let toto = warp::path!("toto" / String)
        .map(|name| format!("toto, {}!", name));

    let routes = warp::get().and(
        hello.or(toto)
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
