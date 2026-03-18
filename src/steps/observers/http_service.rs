use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
};
use std::sync::Arc;

use crate::steps::observers::TieredCache;

pub struct FileHttpService {
    cache: Arc<TieredCache>,
}

impl FileHttpService {
    pub fn new(cache: Arc<TieredCache>) -> Self {
        Self { cache }
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        let app = axum::Router::new()
            .route("/files", axum::routing::get(list_files))
            .route("/files/:path", axum::routing::get(get_file))
            .route("/files/:path/metadata", axum::routing::get(get_metadata))
            .route("/files/:path", axum::routing::head(head_file))
            .with_state(self.cache.clone());

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        println!("File HTTP service listening on port {}", port);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn get_file(
    State(cache): State<Arc<TieredCache>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match cache.get(&path).await {
        Some(content) => {
            let mut headers = HeaderMap::new();
            headers.insert("content-type", content.metadata.mime_type.parse().unwrap());
            headers.insert("content-length", content.metadata.size.to_string().parse().unwrap());
            (StatusCode::OK, headers, content.content).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn list_files(State(cache): State<Arc<TieredCache>>) -> impl IntoResponse {
    let files = cache.list().await;
    Json(files).into_response()
}

async fn get_metadata(
    State(cache): State<Arc<TieredCache>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match cache.get(&path).await {
        Some(content) => Json(content.metadata).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn head_file(
    State(cache): State<Arc<TieredCache>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match cache.get(&path).await {
        Some(content) => {
            let mut headers = HeaderMap::new();
            headers.insert("content-length", content.metadata.size.to_string().parse().unwrap());
            (StatusCode::OK, headers).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
