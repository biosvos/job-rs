mod sourcer;

use sourcer::programmers::Programmers;
use sourcer::sourcer::Sourcer;

extern crate glob;

use glob::glob;
use serde_json::Value;
use clap::Parser;

#[derive(Debug)]
struct Company {
    id: u64,
    name: String,
    address: String,
    jobs: Vec<Job>,
}

impl Company {
    fn add_job(&mut self, job: Job) -> Result<(), Box<dyn std::error::Error>> {
        self.jobs.push(job);
        Ok(())
    }
}

#[derive(Debug)]
struct Job {
    id: u64,
    title: String,
    url: String, // https://career.programmers.co.kr/job_positions/{id}
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long, short, action)]
    refresh: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();

    if args.refresh {
        Programmers.source()?;
    }

    let mut companies = std::collections::HashMap::new();
    for entry in glob("*.json")? {
        let body = std::fs::read_to_string(entry?.as_path())?;
        let json: Value = serde_json::from_str(&body)?;
        for vec in json["jobPositions"].as_array().unwrap() {
            let key = vec["company"]["id"].as_u64().unwrap();
            if !companies.contains_key(&key) {
                companies.insert(key, Company {
                    id: key,
                    name: vec["company"]["name"].as_str().unwrap().into(),
                    address: vec["company"]["address"].as_str().unwrap().into(),
                    jobs: Vec::new(),
                });
            }
            let url = format!("https://career.programmers.co.kr/job_positions/{}", vec["id"].as_u64().unwrap());
            companies.entry(key).and_modify(|company| {
                company.add_job(Job {
                    id: vec["id"].as_u64().unwrap(),
                    title: vec["title"].as_str().unwrap().into(),
                    url,
                }).unwrap();
            });
        }
    }

    dbg!(companies);

    Ok(())
}