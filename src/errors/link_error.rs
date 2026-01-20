use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LinkError {
    LinkNotFoundByCode { code: String },
    LinkNotFoundById { id: String },
    ConflictOnCreate { target: String },
    CouldNotGenerateCode
}