//! HTTP server: the endpoint the Outlook plugin calls.
//!
//!   POST /toll   { "emails": [...], "duration_minutes": 60 }  -> TollReport (JSON)
//!   GET  /health -> "ok"

use std::sync::Arc;

use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Deserialize;

use crate::directory::DirectoryProvider;
use crate::meeting;
use crate::salary::SalaryBook;
use crate::toll::TollReport;

const BIND_ADDR: &str = "127.0.0.1:3000";

struct AppState<D> {
    dir: D,
    book: SalaryBook,
}

#[derive(Deserialize)]
struct TollRequest {
    emails: Vec<String>,
    duration_minutes: u32,
}

/// Start the HTTP server. Swap `dir` to change directory backends.
pub async fn run<D>(dir: D, book: SalaryBook) -> Result<()>
where
    D: DirectoryProvider + Send + Sync + 'static,
{
    let state = Arc::new(AppState { dir, book });

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/toll", post(toll_handler::<D>))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(BIND_ADDR).await?;
    println!("meeting-toll listening on http://{BIND_ADDR}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn toll_handler<D>(
    State(state): State<Arc<AppState<D>>>,
    Json(req): Json<TollRequest>,
) -> Result<Json<TollReport>, AppError>
where
    D: DirectoryProvider + Send + Sync + 'static,
{
    let emails: Vec<&str> = req.emails.iter().map(String::as_str).collect();
    let report = meeting::compute_toll(&state.dir, &state.book, &emails, req.duration_minutes).await?;
    println!("{report}"); // server-side log
    Ok(Json(report))
}

/// Wraps any error as a 500 response.
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
