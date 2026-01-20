use crate::models::link::Link;
pub(crate) use crate::persistence::storage::Storage;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::errors::link_error::LinkError;

pub struct InMemoryStore {
    links: HashMap<Uuid, Link>,
    code_to_id: HashMap<String, Uuid>,
    unique_targets: HashSet<String>,
}

impl InMemoryStore {
    pub fn new() -> InMemoryStore {
        InMemoryStore {
            links: HashMap::new(),
            code_to_id: HashMap::new(),
            unique_targets: HashSet::new(),
        }
    }
}

impl Storage<Link> for InMemoryStore {
    fn load_all(&self) -> Vec<&Link> {
        self.links.values().collect()
    }

    fn store(&mut self, value: Link) -> Result<Link, LinkError> {
        let target = value.target_url.clone();
        if !self.unique_targets.insert(target.clone()) {
            return Err(LinkError::ConflictOnCreate { target });
        }

        let code = value.code.clone();
        let id = value.id.clone();
        self.links.insert(id, value.clone());
        self.code_to_id.insert(code, id);
        Ok(value)
    }

    fn get_by_id(&self, id: &Uuid) -> Option<&Link> {
        self.links.get(id)
    }

    fn get_by_code(&mut self, code: &str) -> Option<&Link> {
        let id = self.code_to_id.get(code);
        match id {
            Some(id) => self.links.get(id),
            None => None,
        }
    }

    fn get_by_code_mut(&mut self, code: &str) -> Option<&mut Link> {
        let id = self.code_to_id.get(code)?;
        self.links.get_mut(id)
    }
}
