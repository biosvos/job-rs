use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Graph {
    map: HashMap<String, String>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            map: Default::default(),
        }
    }

    fn add(&mut self, from: &str, to: &str) {
        self.map.insert(from.to_string(), to.to_string());
    }

    fn keys(&self) -> Keys<'_, String, String> {
        self.map.keys()
    }

    fn get(&self, key: &str) -> Option<String> {
        let x = self.map.get(key);
        match x {
            None => {
                None
            }
            Some(value) => {
                Some(value.to_string())
            }
        }
    }
}


#[derive(Debug)]
pub struct Tags {
    grp: Graph,
    items: Vec<String>,
    originals: HashMap<String, String>,
}

impl Tags {
    pub fn new() -> Tags {
        Tags {
            grp: Graph::new(),
            items: vec![],
            originals: Default::default(),
        }
    }

    pub fn load(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stack: Vec<String> = Vec::new();
        let file = File::open(filename)?;
        let lines = BufReader::new(file).lines();
        for line in lines {
            let line = line?;
            let tab_count = line.chars().take_while(|x| *x == '\t').count();
            let line = &line[tab_count..];
            while stack.len() > tab_count {
                stack.pop();
            }
            self.grp.add(line, stack.last().unwrap_or(&"".to_string()));
            stack.push(line.to_string());
        }

        // for item
        let mut items: Vec<String> = Vec::new();
        let mut originals: HashMap<String, String> = HashMap::new();
        for key in self.keys() {
            let splits = key.split("|").collect::<Vec<&str>>();
            for split in splits {
                originals.insert(String::from(split), String::from(key));
                items.push(String::from(split));
            }
        }
        items.sort_by(|a, b| a.len().cmp(&b.len()));
        items.reverse();

        self.items = items;
        self.originals = originals;

        Ok(())
    }

    pub fn path(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut stack: Vec<String> = Vec::new();
        stack.push(key.to_string());
        while let Some(value) = self.grp.get(stack.last().unwrap()) {
            if value == "" {
                break;
            }
            stack.push(value);
        }
        stack.reverse();
        let ret = stack.iter().map(|x| x.split("|").collect::<Vec<&str>>()[0]).collect::<Vec<&str>>();

        Ok(ret.join("/"))
    }

    pub fn keys(&self) -> Keys<'_, String, String> {
        self.grp.keys()
    }

    pub fn get_tags(&self, paragraph: &str) -> Option<Vec<String>> {
        let mut paragraph = paragraph.to_string();
        let mut ret: Vec<String> = Vec::new();
        for tag in &self.items {
            let original_size = paragraph.len();
            paragraph = paragraph.replace(tag, "");
            if original_size != paragraph.len() {
                ret.push(self.path(self.originals.get(tag).unwrap()).unwrap());
            }
        }

        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tag::Tags;

    #[test]
    fn add() -> Result<(), Box<dyn std::error::Error>> {
        let mut tags = Tags::new();
        tags.load("tagging").unwrap();
        let res = tags.path("Go|go")?;
        println!("{}", res);
        dbg!(tags.keys());
        Ok(())
    }

    #[test]
    fn string_check() {
        let mut tags = Tags::new();
        tags.load("tagging").unwrap();

        let like = "C/C++??? ????????????, ?????? ?????? (Golang)??? ???????????? ?????? ???????????????.".to_string();
        dbg!(tags.get_tags(&like));
    }
}