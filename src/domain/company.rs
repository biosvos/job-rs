use std::fmt::Formatter;
use crate::domain::job::Job;

pub struct Company {
    pub id: u64,
    pub name: String,
    pub address: String,
    pub jobs: Vec<Job>,
}

impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n# {}", self.name)?;
        for job in &self.jobs {
            writeln!(f, "{}", job)?;
        }
        Ok(())
    }
}

impl Company {
    pub fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
    }
}