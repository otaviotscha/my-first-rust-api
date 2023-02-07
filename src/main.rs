use axum::{
    body::boxed,
    http::{header, HeaderValue},
    routing::get,
    Router,
};
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, ServiceBuilderExt};

#[derive(Clone, Debug)]
struct AppState {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .expect("server error");
}

fn app() -> Router {
    // Shared state.
    let state = AppState {};

    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

    let middleware = ServiceBuilder::new()
        .sensitive_request_headers(sensitive_headers.clone())
        .sensitive_response_headers(sensitive_headers.clone())
        .trace_for_http()
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .map_response_body(boxed)
        .compression()
        .insert_response_header_if_not_present(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

    return Router::new()
        .route("/", get(root))
        .layer(middleware)
        .with_state(state);
}

async fn root() -> &'static str {
    "Hello, World!"
}
