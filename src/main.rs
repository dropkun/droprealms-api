mod handler;

use axum::{
    routing::{get, post},
    Router,
};

async fn root() -> &'static str {
    "Welcome to droprealms!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/instance/start", post(handler::start_instance))
        .route("/instance/stop", post(handler::stop_instance))
        .route("/instance/ip", post(handler::get_ip))
        .route("/instance/status", post(handler::get_status));

    println!("droprealms api is ready!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
