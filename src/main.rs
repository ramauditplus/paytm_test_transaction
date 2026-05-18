use axum::{
    Router,
    routing::{get, post},
};

mod api;

use api::{
    create_qr_code::create_qr_code,
    transaction_status::transaction_status,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/createqrcode", post(create_qr_code))
        .route("/transactionstatus", post(transaction_status));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
