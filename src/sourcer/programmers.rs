use std::io::Write;
use crate::sourcer::sourcer::Sourcer;

pub struct Programmers;

impl Programmers {
    fn http_request(&self, page: u64) -> Result<String, Box<dyn std::error::Error>> {
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

    fn write_all(&self, filename: String, body: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mut output = std::fs::File::create(filename)?;
        output.write_all(body.as_bytes())?;
        Ok(())
    }
}

impl Sourcer for Programmers {
    fn source(&self) -> Result<(), Box<dyn std::error::Error>> {
        let body = self.http_request(1)?;
        self.write_all(format!("{}.json", 1), &body)?;

        let value: serde_json::Value = serde_json::from_str(&body)?;
        println!("{}", value["totalPages"]);

        for page in 2..=(value["totalPages"].as_u64().unwrap()) {
            let body = self.http_request(page)?;
            self.write_all(format!("{}.json", page), &body)?;
        }

        Ok(())
    }
}