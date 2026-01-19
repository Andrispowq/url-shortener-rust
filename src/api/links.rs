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
) -> Result<(StatusCode, Json<LinkDto>), (StatusCode, String)> {
    let mut service = state.lock().await;
    service
        .create_link(payload)
        .map(|dto| (StatusCode::CREATED, Json(dto)))
        .map_err(|err| (StatusCode::CONFLICT, err))
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
) -> Result<impl IntoResponse, StatusCode> {
    let mut service = state.lock().await;
    match service.visit_link(code) {
        Ok(link) => Ok(Redirect::temporary(&link.target_url)),
        Err(err) if err == "No such link" => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
