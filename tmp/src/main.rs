// extern crate pretty_env_logger;
// #[macro_use]
// extern crate log;
use std::convert::Infallible;
use std::env;
use tokio::task;
use warp::Filter;

async fn server_1() {
    log::info!("Hello from server 1");
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

async fn server_2() {
    log::info!("Hello from server 2");
    // let hello = warp::path!("hello2" / String).map(|name| format!("Hello, {}!", name));

    let api = stuff;
    let routes = api().with(warp::log("info"));
    warp::serve(routes).run(([127, 0, 0, 1], 4030)).await;
}

pub fn stuff() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    stuff_list()
}

pub async fn list_stuff() -> Result<impl warp::Reply, Infallible> {
    let stuff: Vec<&str> = vec!["first", "second", "third"];
    Ok(warp::reply::json(&stuff))
}

pub fn stuff_list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("stuff").and(warp::get()).and_then(list_stuff)
}

async fn start_client() {
    // let resp1 = task::spawn(server_1());
    let resp2 = task::spawn(server_2());
    // let _ = resp1.await;
    let _ = resp2.await;
}

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init_timed();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(start_client());
}
