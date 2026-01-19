use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLinkDto {
    pub target: String,
}
