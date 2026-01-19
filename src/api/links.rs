use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use tokio::sync::Mutex;

use crate::dto::create_link_dto::CreateLinkDto;
use crate::dto::link_dto::LinkDto;
use crate::service::link_service::{LinkService, LinkServiceTrait};

pub type LinkServiceState = Arc<Mutex<LinkService>>;

pub async fn create_link(
    State(state): State<LinkServiceState>,
    Json(payload): Json<CreateLinkDto>,
) -> Result<Json<LinkDto>, (StatusCode, String)> {
    let mut service = state.lock().await;
    service
        .create_link(payload)
        .map(Json)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))
}

pub async fn visit_link(
    State(state): State<LinkServiceState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut service = state.lock().await;
    match service.visit_link(code) {
        Ok(link) => Ok(Redirect::temporary(&link.target_url)),
        Err(err) if err == "No such link" => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
