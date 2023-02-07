mod sourcer;
mod printer;
mod file_printer;
mod console_printer;

use std::fs::File;
use std::io::Write;
use sourcer::programmers::Programmers;
use sourcer::sourcer::Sourcer;

use clap::Parser;
use crate::console_printer::ConsolePrinter;
use crate::file_printer::FilePrinter;
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

fn contains(arr: &Vec<String>, target: &String) -> bool {
    for x in arr {
        if x.contains(target) {
            return true;
        }
    }
    return false;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Cli = Cli::parse();

    if args.refresh {
        Programmers.source()?;
    }

    let mut companies = Programmers.parse()?;

    for company in &mut companies {
        for filter in args.excludes.iter_mut() {
            company.jobs.retain(|job| !job.title.contains(filter.as_str()));
            company.jobs.retain(|job| !contains(&job.requirements, filter));
        }
    }
    companies.retain(|company| company.jobs.len() > 0);

    let mut printer: Box<dyn Printer> = if let Some(filename) = args.output {
        FilePrinter::new(filename)?
    } else {
        ConsolePrinter::new()
    };

    let mut counter = 0;
    for company in &mut companies {
        printer.println(format!("\n# {}", company.name))?;
        for job in company.jobs.iter_mut() {
            printer.println(format!("- [{}]({})", job.title, job.url))?;
            counter += 1;
            for requirement in job.requirements.iter_mut() {
                let re = regex::Regex::new("<.+?>")?;
                let paragraph = re.replace_all(requirement, "");
                let paragraph = paragraph.replace('\u{a0}', "");
                let paragraph = paragraph.replace('\\', "");
                if !paragraph.is_empty() {
                    printer.println(format!("  - {}", paragraph))?;
                }
            }
        }
    }

    eprintln!("{}", counter);
    Ok(())
}