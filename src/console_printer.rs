use std::error::Error;
use crate::printer::Printer;

pub struct ConsolePrinter {}

impl Printer for ConsolePrinter {
    fn println(&mut self, string: String) -> Result<(), Box<dyn Error>> {
        println!("{}", string);
        Ok(())
    }
}

impl ConsolePrinter {
    pub fn new() -> Box<dyn Printer> {
        Box::new(ConsolePrinter {})
    }
}