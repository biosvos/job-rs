use std::fmt::Formatter;

pub struct Job {
    pub id: u64,
    pub title: String,
    pub url: String,
    pub requirements: Vec<String>,
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- [{}]({})", self.title, self.url)?;
        for requirement in &self.requirements {
            writeln!(f, "  - {}", requirement)?;
        }
        Ok(())
    }
}

impl Job {
    pub fn contain(&self, like: &str) -> bool {
        if self.title.contains(like) {
            return true;
        }
        for requirement in &self.requirements {
            if requirement.contains(like) {
                return true;
            }
        }
        return false;
    }
}