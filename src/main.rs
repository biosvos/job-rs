mod sourcer;

use sourcer::programmers::Programmers;
use sourcer::sourcer::Sourcer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Programmers.source()?;

    Ok(())
}