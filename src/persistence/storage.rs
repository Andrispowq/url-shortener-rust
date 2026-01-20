use uuid::Uuid;
use crate::errors::link_error::LinkError;

pub trait Storage<T> {
    fn load_all(&self) -> Vec<&T>;
    fn store(&mut self, value: T) -> Result<T, LinkError>;

    fn get_by_id(&self, id: &Uuid) -> Option<&T>;
    fn get_by_code(&mut self, code: &str) -> Option<&T>;
    fn get_by_code_mut(&mut self, code: &str) -> Option<&mut T>;
}
