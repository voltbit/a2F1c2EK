use std::env;

use tokio::task;

mod external_api;
mod internal_api;
mod models;

async fn start_client() {
    let external = task::spawn(external_api::external_server());
    // let internal = task::spawn(internal_api::internal_server());
    external.await.expect("Data client external server");
    // internal.await.expect("Data client internal server");
}

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init_timed();

    log::info!("Starting user client");
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(start_client());
}
