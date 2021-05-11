use std::env;

use tokio::task;

mod external_api;
mod internal_api;

async fn start_client() {
    let external = task::spawn(external_api::external_server());
    let internal = task::spawn(internal_api::internal_server());
    let _ = external.await;
    let _ = internal.await;
}

fn main() {
    println!("Starting user client");
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init_timed();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(start_client());
}
