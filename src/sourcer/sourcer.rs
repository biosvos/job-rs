use crate::domain::company::Company;

pub trait Sourcer {
    fn fetch(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn load(&self) -> Result<Vec<Company>, Box<dyn std::error::Error>>;
}
