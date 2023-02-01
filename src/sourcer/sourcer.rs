#[derive(Debug)]
pub struct Company {
    pub id: u64,
    pub name: String,
    pub address: String,
    pub jobs: Vec<Job>,
}

impl Company {
    pub(crate) fn add_job(&mut self, job: Job) -> Result<(), Box<dyn std::error::Error>> {
        self.jobs.push(job);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Job {
    pub id: u64,
    pub title: String,
    pub url: String,
    pub requirements: Vec<String>,
}

pub trait Sourcer {
    fn source(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn parse(&self) -> Result<Vec<Company>, Box<dyn std::error::Error>>;
}
