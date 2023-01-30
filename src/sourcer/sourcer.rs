pub trait Sourcer {
    fn source(&self) -> Result<(), Box<dyn std::error::Error>>;
}
