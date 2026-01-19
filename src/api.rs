use utoipa::OpenApi;

pub mod links;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::links::create_link,
        crate::api::links::visit_link,
        crate::api::links::get_all_links,
    ),
    components(
        schemas(
            crate::dto::create_link_dto::CreateLinkDto,
            crate::dto::link_dto::LinkDto,
        )
    ),
    tags(
        (name = "links", description = "Short-link management endpoints")
    )
)]
pub struct ApiDoc;
