mod sourcer;
mod printer;
mod domain;
mod tag;

use sourcer::programmers::Programmers;
use sourcer::sourcer::Sourcer;

use clap::Parser;
use crate::printer::Printer;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long, short, action)]
    refresh: bool,

    #[clap(long, short, num_args(0..))]
    excludes: Vec<String>,

    #[clap(long, short)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Cli = Cli::parse();

    if args.refresh {
        Programmers.fetch()?;
    }

    let mut companies = Programmers.load()?;

    for company in &mut companies {
        for filter in args.excludes.iter_mut() {
            company.jobs.retain(|job| !job.contain(&filter));
        }
    }
    companies.retain(|company| company.jobs.len() > 0);

    let mut printer: Box<dyn Printer> = if let Some(filename) = args.output {
        printer::FilePrinter::new(filename)?
    } else {
        printer::ConsolePrinter::new()
    };

    let mut counter = 0;
    for company in &mut companies {
        printer.println(format!("{}", company))?;
        counter += company.jobs.len()
    }

    eprintln!("{}", counter);
    Ok(())
}