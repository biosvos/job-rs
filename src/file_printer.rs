use std::error::Error;
use crate::printer::Printer;
use std::io::Write;

pub struct FilePrinter {
    writer: std::fs::File,
}

impl Printer for FilePrinter {
    fn println(&mut self, string: String) -> Result<(), Box<dyn Error>> {
        writeln!(&self.writer, "{}", string)?;
        Ok(())
    }
}

impl FilePrinter {
    pub fn new(filename: String) -> Result<FilePrinter, Box<dyn Error>> {
        let ret = FilePrinter {
            writer: std::fs::File::create(filename)?,
        };
        Ok(ret)
    }
}