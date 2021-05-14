extern crate pretty_env_logger;
use std::env;

use tokio::task;

mod job_controller;

async fn start_controller() {
    let controller = task::spawn(job_controller::controller_server());
    let _ = controller.await;
}

fn main() {
    println!("Starting job controller");
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init_timed();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(start_controller());
    println!("Hello, world!");
}
