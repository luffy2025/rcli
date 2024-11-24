use anyhow::{anyhow, Result};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct HttpServeParam {
    deep: Option<u8>,
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
    Query(params): Query<HttpServeParam>,
) -> impl IntoResponse {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Serving file {:?}", p);

    let expect_deep = params.deep.unwrap_or(0);

    if !p.exists() {
        (StatusCode::NOT_FOUND, format!("File not found: {:?}", p))
    } else if p.is_dir() {
        match tree(p, expect_deep) {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => {
                warn!("Failed to list directory {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to list directory".to_string(),
                )
            }
        }
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

fn tree(path: PathBuf, expect_deep: u8) -> Result<String> {
    if !path.is_dir() {
        return Err(anyhow!("{} is not a directory", path.display()));
    }
    let mut buf = String::new();
    buf.push_str("\n------------------ use std::fs ----------------------\n");
    // use std::fs;
    buf.push_str(format_filename(&path, "")?.as_str());
    walk_dir_with_std(&mut buf, path.clone(), 1, expect_deep)?;

    buf.push_str("\n---------------- use walkdir::WalkDir ----------------\n");

    // use walkdir::WalkDir;
    walk_dir_with_walkdir(&mut buf, path, expect_deep)?;
    Ok(buf)
}

fn walk_dir_with_std(buf: &mut String, path: PathBuf, deep: u8, expect_deep: u8) -> Result<()> {
    if expect_deep > 0 && deep > expect_deep {
        return Ok(());
    }

    if !path.is_dir() {
        return Ok(());
    }

    for entry in (fs::read_dir(path)?).flatten() {
        let path = entry.path();
        if path.is_dir() {
            let indent = "  ".repeat(deep as usize);
            buf.push_str(format_filename(&path, &indent)?.as_str());
            walk_dir_with_std(buf, entry.path(), deep + 1, expect_deep)?;
        } else {
            let indent = "  ".repeat(deep as usize);
            buf.push_str(format_filename(&path, &indent)?.as_str());
        }
    }

    Ok(())
}

fn walk_dir_with_walkdir(buf: &mut String, path: PathBuf, expect_deep: u8) -> Result<()> {
    let walker = if expect_deep > 0 {
        walkdir::WalkDir::new(path).max_depth(expect_deep as _)
    } else {
        walkdir::WalkDir::new(path)
    };
    for entry in walker.into_iter().flatten() {
        let indent = "  ".repeat(entry.depth());
        buf.push_str(format_filename(entry.path(), &indent)?.as_str());
    }
    Ok(())
}

fn format_filename(path: &std::path::Path, indent: &str) -> Result<String> {
    if let Some(name) = path.file_name() {
        Ok(format!("{}|--{}\n", indent, name.to_string_lossy()))
    } else {
        Err(anyhow!("Failed to get file name"))
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
        let (header, _) = file_handler(
            State(state),
            Path("Cargo.toml".to_string()),
            Query(HttpServeParam { deep: None }),
        )
        .await
        .into_response()
        .into_parts();
        assert_eq!(header.status, StatusCode::OK);
    }
}
