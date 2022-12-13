use axum::Router;
use std::net::SocketAddr;

pub mod controllers;
pub mod services;

use crate::controllers::root::register_root_controller;

#[tokio::main]
async fn main() {
    // build our application
    let mut app = Router::new();

    // add root route
    app = register_root_controller(app).await;

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
