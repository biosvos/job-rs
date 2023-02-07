use crate::domain::company::Company;

pub trait Sourcer {
    fn source(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn parse(&self) -> Result<Vec<Company>, Box<dyn std::error::Error>>;
}
