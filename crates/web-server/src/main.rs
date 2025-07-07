use std::net::SocketAddr;

use axum::{Extension, Router, routing::get};
use tower_livereload::LiveReloadLayer;

mod config;
mod error;
mod root;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.db_url);

    let app = Router::new()
        .route("/", get(root::loader))
        .layer(LiveReloadLayer::new())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}
