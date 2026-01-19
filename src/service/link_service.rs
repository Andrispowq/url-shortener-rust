use crate::dto::link_dto::Dto;
use crate::dto::create_link_dto::CreateLinkDto;
use crate::dto::link_dto::LinkDto;
use crate::models::link::Link;
use crate::persistence::in_memory_store::{InMemoryStore, Storage};

pub trait LinkServiceTrait {
    fn create_link(&mut self, dto: CreateLinkDto) -> Result<LinkDto, String>;
    fn visit_link(&mut self, code: String) -> Result<LinkDto, String>;
}

pub struct LinkService {
    store: InMemoryStore
}

impl LinkServiceTrait for LinkService {
    fn create_link(&mut self, dto: CreateLinkDto) -> Result<LinkDto, String> {
        let link = Link::new(String::from("todo"), dto.target, 0);
        let dto = link.to_dto();
        self.store.store(link);
        Ok(dto)
    }

    fn visit_link(&mut self, code: String) -> Result<LinkDto, String> {
        let link = self.store.get_by_code(&code);
        if link.is_none() {
            return Err(String::from("No such link"))
        }
        
        let link = link.unwrap();
        let increased = link.increased_clicks();
        
        let dto = increased.to_dto();
        Ok(dto)
    }
}