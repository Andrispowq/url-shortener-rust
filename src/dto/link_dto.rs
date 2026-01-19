pub(crate) use crate::dto::dto::Dto;
use crate::models::link::Link;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkDto {
    pub code: String,
    pub target_url: String,
    pub created_at: DateTime<Utc>,
    pub clicks: u64,
}

impl Dto<LinkDto> for Link {
    fn to_dto(&self) -> LinkDto {
        LinkDto {
            code: self.code.clone(),
            target_url: self.target_url.clone(),
            created_at: self.created_at,
            clicks: self.clicks,
        }
    }
}
