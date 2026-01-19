pub(crate) use crate::dto::dto::Dto;
use crate::models::link::Link;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LinkDto {
    pub id: String,
    pub code: String,
    pub target_url: String,
    pub created_at: DateTime<Utc>,
    pub clicks: u64,
}

impl Dto<LinkDto> for Link {
    fn to_dto(&self) -> LinkDto {
        LinkDto {
            id: self.id.to_string(),
            code: self.code.clone(),
            target_url: self.target_url.clone(),
            created_at: self.created_at,
            clicks: self.clicks,
        }
    }
}
