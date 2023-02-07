mod sourcer;

use std::fs::File;
use std::io::Write;
use sourcer::programmers::Programmers;
use sourcer::sourcer::Sourcer;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long, short, action)]
    refresh: bool,

    #[clap(long, short, num_args(0..))]
    excludes: Vec<String>,

    #[clap(long, short)]
    output: String,
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

    let mut writer: File;
    writer = File::create(args.output)?;

    let mut counter = 0;
    for company in &mut companies {
        writeln!(&mut writer, "\n# {}", company.name)?;
        for job in company.jobs.iter_mut() {
            writeln!(&mut writer, "- [{}]({})", job.title, job.url)?;
            counter += 1;
            for requirement in job.requirements.iter_mut() {
                let re = regex::Regex::new("<.+?>")?;
                let paragraph = re.replace_all(requirement, "");
                let paragraph = paragraph.replace('\u{a0}', "");
                let paragraph = paragraph.replace('\\', "");
                if !paragraph.is_empty() {
                    writeln!(&mut writer, "  - {}", paragraph)?;
                }
            }
        }
    }

    eprintln!("{}", counter);
    Ok(())
}