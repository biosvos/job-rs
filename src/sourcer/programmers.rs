use std::error::Error;
use std::io::Write;
use crate::domain::company::Company;
use crate::domain::job::Job;
use crate::sourcer::sourcer::Sourcer;

pub struct Programmers;

impl Programmers {
    fn list_jobs(&self, page: u64) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://career.programmers.co.kr/api/job_positions";
        let params = [
            ("min_salary", "6000"),
            ("order", "recent"),
            ("page", &page.to_string()),
            ("job_category_ids[]", "1")
        ];
        let url = reqwest::Url::parse_with_params(url, &params)?;
        let body = reqwest::blocking::get(url)?.text()?;
        Ok(body)
    }

    fn get_job(&self, job_id: u64) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("https://career.programmers.co.kr/api/job_positions/{}", job_id);
        let body = reqwest::blocking::get(url)?.text()?;
        Ok(body)
    }

    fn write_all(&self, filename: String, body: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mut output = std::fs::File::create(filename)?;
        output.write_all(body.as_bytes())?;
        Ok(())
    }
}

impl Sourcer for Programmers {
    fn fetch(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all("pages")?;
        std::fs::create_dir_all("details")?;

        let body = self.list_jobs(1)?;
        self.write_all(format!("pages/{}.json", 1), &body)?;

        let value: serde_json::Value = serde_json::from_str(&body)?;

        for page in 2..=(value["totalPages"].as_u64().unwrap()) {
            let body = self.list_jobs(page)?;
            self.write_all(format!("pages/{}.json", page), &body)?;
        }

        for entry in glob::glob("pages/*.json")? {
            let body = std::fs::read_to_string(entry?.as_path())?;
            let json: serde_json::Value = serde_json::from_str(&body)?;
            for vec in json["jobPositions"].as_array().unwrap() {
                let job_id = vec["id"].as_u64().unwrap();
                let body = self.get_job(job_id)?;
                self.write_all(format!("details/{}.json", job_id), &body)?;
            }
        }

        Ok(())
    }

    fn load(&self) -> Result<Vec<Company>, Box<dyn Error>> {
        let mut companies = std::collections::HashMap::new();
        for entry in glob::glob("pages/*.json")? {
            let body = std::fs::read_to_string(entry?.as_path())?;
            let json: serde_json::Value = serde_json::from_str(&body)?;
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
                let job_id = vec["id"].as_u64().unwrap();
                let title = vec["title"].as_str().unwrap().into();

                let detail = std::fs::read_to_string(format!("details/{}.json", job_id))?;
                let detail_json: serde_json::Value = serde_json::from_str(&detail)?;
                let requirement: String = detail_json["jobPosition"]["requirement"].as_str().unwrap_or("").into();
                let requirements = requirement.split("\r\n").map(|s| {
                    let re = regex::Regex::new("<.+?>").unwrap();
                    let paragraph = re.replace_all(s, "");
                    let paragraph = paragraph.replace('\u{a0}', " ");
                    let paragraph = paragraph.replace('\\', "");
                    paragraph
                }).filter(|s| !s.is_empty()).collect();

                let url = format!("https://career.programmers.co.kr/job_positions/{}", job_id);
                companies.entry(key).and_modify(|company| {
                    company.add_job(Job {
                        id: job_id,
                        title,
                        url,
                        requirements,
                    });
                });
            }
        }

        Ok(companies.into_values().collect())
    }
}