mod sourcer;

use sourcer::programmers::Programmers;
use sourcer::sourcer::Sourcer;

extern crate glob;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long, short, action)]
    refresh: bool,

    #[clap(long, short, num_args(0..))]
    excludes: Vec<String>,
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
        }
    }
    companies.retain(|company| company.jobs.len() > 0);

    for company in &mut companies {
        println!("{}", company.name);
        for job in company.jobs.iter_mut() {
            println!("- [{}]({})", job.title, job.url);
        }
    }

    Ok(())
}