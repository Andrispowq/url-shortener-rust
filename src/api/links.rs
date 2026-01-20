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
use crate::errors::link_error::LinkError;
use crate::service::link_service::{LinkService, LinkServiceTrait};

type ErrorResponse = (StatusCode, String);

pub type LinkServiceState = Arc<Mutex<LinkService>>;

#[utoipa::path(
    post,
    path = "/links",
    request_body = CreateLinkDto,
    responses(
        (status = 201, description = "Short link created", body = LinkDto),
        (status = 500, description = "Internal error", body = String),
    ),
    tag = "links"
)]
pub async fn create_link(
    State(state): State<LinkServiceState>,
    Json(payload): Json<CreateLinkDto>,
) -> Result<(StatusCode, Json<LinkDto>), ErrorResponse> {
    let mut service = state.lock().await;
    service
        .create_link(payload)
        .map(|dto| (StatusCode::CREATED, Json(dto)))
        .map_err(|err| err.to_response())
}

#[utoipa::path(
    get,
    path = "/r/{code}",
    params(
        ("code" = String, Path, description = "Short code to resolve"),
    ),
    responses(
        (status = 307, description = "Redirect to target URL"),
        (status = 404, description = "Short link not found"),
        (status = 500, description = "Internal error"),
    ),
    tag = "links"
)]
pub async fn visit_link(
    State(state): State<LinkServiceState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut service = state.lock().await;
    match service.visit_link(code) {
        Ok(link) => Ok(Redirect::temporary(&link.target_url)),
        Err(err) => Err(err.to_response())
    }
}

#[utoipa::path(
    get,
    path = "/links",
    responses(
        (status = 200, description = "A list of links", body = Vec<LinkDto>),
    ),
    tag = "links"
)]
pub async fn get_all_links(
    State(state): State<LinkServiceState>
) -> (StatusCode, Json<Vec<LinkDto>>) {
    let service = state.lock().await;
    (StatusCode::OK, Json::from(service.get_all_links()))
}

impl LinkError {
    pub fn to_response(&self) -> ErrorResponse {
        match self {
            LinkError::LinkNotFoundByCode { code } => (StatusCode::NOT_FOUND, format!("Link with code {} not found", code)),
            LinkError::LinkNotFoundById { id } => (StatusCode::NOT_FOUND, format!("Link with id {} not found", id)),
            LinkError::ConflictOnCreate { target } => (StatusCode::CONFLICT, format!("Target \"{}\" already exists", target)),
            LinkError::CouldNotGenerateCode => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Could not generate code")),
        }
    }
}