mod api;
mod dto;
mod models;
mod persistence;
mod service;
mod errors;

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::ApiDoc;
use crate::api::links::{LinkServiceState, create_link, visit_link, get_all_links};
use crate::persistence::in_memory_store::InMemoryStore;
use crate::service::code_generator::CodeGenerator;
use crate::service::link_service::LinkService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state: LinkServiceState = Arc::new(Mutex::new(LinkService::new(
        InMemoryStore::new(),
        CodeGenerator::new(6),
    )));

    let app = Router::new()
        .route("/links", post(create_link))
        .route("/r/{code}", get(visit_link))
        .route("/links", get(get_all_links))
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind listener");
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server failed");
}
