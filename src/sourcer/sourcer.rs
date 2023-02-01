#[derive(Debug)]
pub struct Company {
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
pub struct Job {
    id: u64,
    title: String,
    url: String, // https://career.programmers.co.kr/job_positions/{id}
}

pub trait Sourcer {
    fn source(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn Parse(&self) -> Result<Vec<Company>, Box<dyn std::error::Error>>;
}
