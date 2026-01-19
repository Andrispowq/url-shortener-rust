use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Link {
    pub id: Uuid,
    pub code: String,
    pub target_url: String,
    pub created_at: DateTime<Utc>,
    pub clicks: u64
}

impl Link {
    pub fn new(code: String, target_url: String, clicks: u64) -> Link {
        Link {
            id: Uuid::now_v7(),
            code,
            target_url,
            created_at: Utc::now(),
            clicks: 0
        }
    }
    pub fn increased_clicks(self) -> Link {
        Link {
            id: self.id,
            code: self.code,
            target_url: self.target_url,
            created_at: self.created_at,
            clicks: self.clicks + 1
        }
    }
}