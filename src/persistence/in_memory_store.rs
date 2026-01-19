use crate::models::link::Link;
pub(crate) use crate::persistence::storage::Storage;
use std::collections::HashMap;
use uuid::Uuid;

pub struct InMemoryStore {
    links: HashMap<Uuid, Link>,
    code_to_id: HashMap<String, Uuid>,
}

impl InMemoryStore {
    pub fn new() -> InMemoryStore {
        InMemoryStore {
            links: HashMap::new(),
            code_to_id: HashMap::new(),
        }
    }
}

impl Storage<Link> for InMemoryStore {
    fn load_all(&self) -> Vec<&Link> {
        self.links.values().collect()
    }

    fn store(&mut self, value: Link) {
        let code = value.code.clone();
        let id = value.id.clone();
        self.links.insert(id, value);
        self.code_to_id.insert(code, id);
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
