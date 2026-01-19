use crate::dto::create_link_dto::CreateLinkDto;
use crate::dto::link_dto::Dto;
use crate::dto::link_dto::LinkDto;
use crate::models::link::Link;
use crate::persistence::in_memory_store::{InMemoryStore, Storage};
use crate::service::code_generator::CodeGenerator;

pub trait LinkServiceTrait {
    fn get_all_links(&self) -> Vec<LinkDto>;
    fn create_link(&mut self, dto: CreateLinkDto) -> Result<LinkDto, String>;
    fn visit_link(&mut self, code: String) -> Result<LinkDto, String>;
}

pub struct LinkService {
    store: InMemoryStore,
    generator: CodeGenerator,
}

impl LinkService {
    pub fn new(store: InMemoryStore, generator: CodeGenerator) -> LinkService {
        LinkService { store, generator }
    }
}

impl LinkServiceTrait for LinkService {
    fn get_all_links(&self) -> Vec<LinkDto> {
        self.store.load_all().iter().map(|l| l.to_dto()).collect()
    }
    
    fn create_link(&mut self, dto: CreateLinkDto) -> Result<LinkDto, String> {
        let link = Link::new(self.generator.generate()?, dto.target);
        let result = self.store.store(link)?;
        Ok(result.to_dto())
    }

    fn visit_link(&mut self, code: String) -> Result<LinkDto, String> {
        let link = self
            .store
            .get_by_code_mut(&code)
            .ok_or_else(|| "No such link".to_string())?;
        link.increase_clicks();
        Ok(link.to_dto())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_service(length: usize) -> LinkService {
        LinkService::new(InMemoryStore::new(), CodeGenerator::new(length))
    }

    fn make_create_dto(target: &str) -> CreateLinkDto {
        CreateLinkDto {
            target: target.to_string(),
        }
    }

    #[test]
    fn create_link_returns_persisted_dto() {
        let mut service = build_service(6);
        let dto = service
            .create_link(make_create_dto("https://example.com"))
            .expect("link should be created");

        assert_eq!(dto.target_url, "https://example.com");
        assert_eq!(dto.clicks, 0);
        assert_eq!(dto.code.len(), 6);
    }

    #[test]
    fn visit_link_increments_click_count() {
        let mut service = build_service(5);
        let created = service
            .create_link(make_create_dto("https://visits.test"))
            .expect("link should be created");
        let code = created.code.clone();

        let first_visit = service
            .visit_link(code.clone())
            .expect("first visit should succeed");
        assert_eq!(first_visit.clicks, 1);

        let second_visit = service
            .visit_link(code)
            .expect("second visit should succeed");
        assert_eq!(second_visit.clicks, 2);
    }

    #[test]
    fn visit_unknown_code_returns_error() {
        let mut service = build_service(4);

        let error = service
            .visit_link("missing".to_string())
            .expect_err("missing code should error");

        assert_eq!(error, "No such link");
    }
}
