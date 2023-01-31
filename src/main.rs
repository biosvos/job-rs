// mod sourcer;
//
// use sourcer::programmers::Programmers;
// use sourcer::sourcer::Sourcer;

extern crate glob;

use glob::glob;
use serde_json::Value;

struct Company {
    id: u64,
    name: String,
    address: String,
    jobs: Vec<Job>,
}

struct Job {
    id: u64,
    title: String,
    url: String, // https://career.programmers.co.kr/job_positions/{id}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Programmers.source()?;
    let mut companies = std::collections::HashMap::new();
    for entry in glob("*.json")? {
        let body = std::fs::read_to_string(entry?.as_path())?;
        let json: Value = serde_json::from_str(&body)?;
        for vec in json["jobPositions"].as_array().unwrap() {
            println!("{}", vec["company"]["id"].as_u64().unwrap());
            companies.insert(vec["company"]["id"].as_u64().unwrap(), Company{
                id: 0,
                name: "".to_string(),
                address: "".to_string(),
                jobs: vec![],
            });
        }
    }

    Ok(())
}