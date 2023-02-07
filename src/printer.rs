pub trait Printer {
    fn println(&mut self, string: String) -> Result<(), Box<dyn std::error::Error>>;
}