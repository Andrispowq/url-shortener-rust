use uuid::Uuid;

pub trait Storage<T: Clone> {
    fn load_all(&self) -> Vec<&T>;
    fn store(&mut self, value: T);

    fn get_by_id(&self, id: &Uuid) -> Option<&T>;
    fn get_by_code(&mut self, code: &String) -> Option<&T>;
    fn get_by_code_mut(&mut self, code: &str) -> Option<&mut T>;
}