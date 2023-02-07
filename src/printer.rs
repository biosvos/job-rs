use std::io::Write;

pub trait Printer {
    fn println(&mut self, string: String) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct FilePrinter {
    writer: std::fs::File,
}

impl Printer for FilePrinter {
    fn println(&mut self, string: String) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(&self.writer, "{}", string)?;
        Ok(())
    }
}

impl FilePrinter {
    pub fn new(filename: String) -> Result<Box<dyn Printer>, Box<dyn std::error::Error>> {
        let ret = FilePrinter {
            writer: std::fs::File::create(filename)?,
        };
        Ok(Box::new(ret))
    }
}

pub struct ConsolePrinter {}

impl Printer for ConsolePrinter {
    fn println(&mut self, string: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", string);
        Ok(())
    }
}

impl ConsolePrinter {
    pub fn new() -> Box<dyn Printer> {
        Box::new(ConsolePrinter {})
    }
}