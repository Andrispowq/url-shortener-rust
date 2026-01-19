
pub trait Dto<Target> {
    fn to_dto(&self) -> Target;
}