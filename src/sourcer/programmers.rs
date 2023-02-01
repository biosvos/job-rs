use std::error::Error;
use std::io::Write;
use crate::sourcer::sourcer::{Company, Sourcer};

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
    fn source(&self) -> Result<(), Box<dyn std::error::Error>> {
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

    fn Parse(&self) -> Result<Vec<Company>, Box<dyn Error>> {
        todo!()
    }
}