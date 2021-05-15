#![allow(unused)]
extern crate pretty_env_logger;
use std::env;

use tokio::task;

mod job_controller;
mod job_db;

async fn start_controller() {
    let controller = task::spawn(job_controller::controller_server());
    let _ = controller.await;
}

fn test_db() {
    let mut db = job_db::build_sync_db();
    println!("DB: {:?}", db);
}

fn main() {
    println!("Starting job controller");
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init_timed();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(start_controller());
    test_db();
}
