use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};

use tower_http::services::ServeDir;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving directory {:?} on addr {}", path, addr);

    let state = HttpServeState { path };
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", ServeDir::new(state.path.clone()))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Serving file {:?}", p);

    if !p.exists() {
        (StatusCode::NOT_FOUND, format!("File not found: {:?}", p))
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Failed to read file {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to read file".to_string(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (header, _) = file_handler(State(state), Path("Cargo.toml".to_string()))
            .await
            .into_response()
            .into_parts();
        assert_eq!(header.status, StatusCode::OK);
    }
}
