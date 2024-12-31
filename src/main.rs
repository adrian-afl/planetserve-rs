use axum::body::Body;
use axum::extract::State;
use axum::response::Response;
use axum::{routing::post, Json, Router};
use clap::Parser;
use serde::Deserialize;
use std::fs::File;
use std::sync::Arc;

use crate::brotli_future_stream::BrotliDecompressStream;
use crate::cli_args::CLIArgs;

mod brotli_future_stream;
mod cli_args;

#[tokio::main]
async fn main() {
    let cli_args = CLIArgs::parse();

    let state = AppState {
        base_path: Arc::from(cli_args.path),
    };

    let app = Router::new()
        .route("/get", post(get_file))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cli_args.port))
        .await
        .unwrap();
    println!("Starting on 0.0.0.0:{}", cli_args.port);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    base_path: Arc<String>,
}

#[derive(Deserialize)]
struct GetFile {
    path: String,
}

async fn get_file(State(state): State<AppState>, Json(payload): Json<GetFile>) -> Response {
    let file = File::open(format!("{}/{}", state.base_path, payload.path).as_str());
    let file = match file {
        Ok(file) => file,
        Err(_) => {
            return Response::builder()
                .status(404)
                .body(Body::from("Not found"))
                .unwrap()
        }
    };
    let decompressor = BrotliDecompressStream::new(brotli::Decompressor::new(file, 40960));
    Response::builder()
        .header("content-type", "application/octet-stream")
        .body(Body::from_stream(decompressor))
        .unwrap()
}
