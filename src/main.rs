use axum::body::Body;
use axum::extract::State;
use axum::response::Response;
use axum::{routing::post, Json, Router};
use clap::Parser;
use serde::Deserialize;
use std::fs::File;
use std::sync::{Arc, Mutex};

use crate::cli_args::CLIArgs;
use crate::future_stream::FutureStream;

mod cli_args;
mod future_stream;

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
    decompress: bool,
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
    match payload.decompress {
        true => {
            let brotli_stream = brotli::Decompressor::new(file, 40960);
            let decompressor = FutureStream::new(Arc::new(Mutex::from(brotli_stream)));
            Response::builder()
                .header("content-type", "application/octet-stream")
                .body(Body::from_stream(decompressor))
                .unwrap()
        }
        false => {
            let decompressor = FutureStream::new(Arc::new(Mutex::from(file)));
            Response::builder()
                .header("content-type", "application/octet-stream")
                .body(Body::from_stream(decompressor))
                .unwrap()
        }
    }
}
