// language: rust
// Path: spell-checker/src/main.rs
use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

pub struct Checker {
    pub misspellings: u32,
    pub text_words: u32,
    dictionary_words: HashSet<String>,
}

impl Checker {
    pub fn new(path: &Path) -> Self {
        
        let mut words = HashSet::new();
        let file = File::open(Path::new(path)).unwrap();
        let buffered = BufReader::new(file);
        for line in buffered.lines() {
            let line = line.unwrap();
            if line.len() > 0 {
                words.insert(line.trim().to_string().to_lowercase());
            }
        }
        // if the dictionary can not load throw an error
        if words.len() == 0 {
            panic!("Could not load dictionary.");
        }
        Checker {
            misspellings: 0,
            text_words: 0,
            dictionary_words: words,
        }
    }

    pub fn len(&self) -> usize {
        self.dictionary_words.len()
    }

    fn check(&self, word: &str) -> bool {
        self.dictionary_words.contains(&word.to_lowercase())
    }

    pub fn check_file(&mut self, path: &Path) {   
        let file = File::open(Path::new(path)).unwrap();
        let buffered = BufReader::new(file);

       
        // scan each char in the file creating a word
        for line in buffered.lines() {
            let line = line.unwrap();
            if line.len() > 0 {
                let mut word = String::new();
                for c in line.chars() {
                    if (c.is_alphanumeric() || c == '\'' || c == '-') && !c.is_numeric() {
                        word.push(c);                 
                    } else {
                        if word.len() > 0 && !word.starts_with('\'') && !word.starts_with('-') {     
                            if word.contains("--") {
                                let words = word.split("--").collect::<Vec<&str>>();
                                for w in words {
                                    if !self.check(w) {
                                        self.misspellings += 1;
                                        println!("{}", w);
                                    }
                                }
                            }                      
                            else if !self.check(&word)  {
                                    self.misspellings += 1;
                                    println!("{}", word);
                    
                            }
                            self.text_words += 1;
                            word = String::new();
                        }
                    }
                }
                if word.len() > 0 && !word.starts_with('\'') && !word.starts_with('-') {
                    if word.contains("--") {
                        let words = word.split("--").collect::<Vec<&str>>();
                        for w in words {
                            if !self.check(w) {
                                self.misspellings += 1;
                                println!("{}", w);
                            }
                        }
                    }               
                    else if !self.check(&word)  {
                        self.misspellings += 1;
                        println!("{}", word);
                    }
                    self.text_words += 1;
                }
            }
        }
    }


}


#[cfg(test)]
mod tests {
    use std::time::Instant;
    use std::path::Path;
    use super::Checker;
   
    #[test]
    fn test_war_and_peace() {
        let dictionary_file: &Path = Path::new("src/data/dictionary.txt");
        let text_file: &Path = Path::new("src/data/wap.txt");


        // if there is a second arg then use that as the dictionary file
        
        // start the timer for load
        let before = Instant::now();
        let mut dictionary = Checker::new(dictionary_file);
        println!("TIME IN LOAD: {:.2}", before.elapsed().as_millis() as f64 / 1000.);
        
        let before = Instant::now();
        dictionary.len();
        println!("TIME IN LENGTH: {:.2}", before.elapsed().as_millis() as f64 / 1000.);

        let before = Instant::now();
        println!("MISSPELLED WORDS");
        println!("-----------------");
        dictionary.check_file(text_file);
        println!("TIME IN CHECK: {:.2}", before.elapsed().as_millis() as f64 / 1000.);
        println!("WORDS IN MISSPELLED: {}", dictionary.misspellings);
        println!("WORDS IN DICTIONARY: {}", dictionary.len());
        println!("WORDS IN TEXT: {}", dictionary.text_words);
    }

    #[test]
    fn test_alice() {
        let dictionary_file: &Path = Path::new("src/data/dictionary.txt");
        let text_file: &Path = Path::new("src/data/alice.txt");


        // if there is a second arg then use that as the dictionary file
        
        // start the timer for load
        let before = Instant::now();
        let mut dictionary = Checker::new(dictionary_file);
        println!("TIME IN LOAD: {:.2}", before.elapsed().as_millis() as f64 / 1000.);
        
        let before = Instant::now();
        dictionary.len();
        println!("TIME IN LENGTH: {:.2}", before.elapsed().as_millis() as f64 / 1000.);

        let before = Instant::now();
        println!("MISSPELLED WORDS");
        println!("-----------------");
        dictionary.check_file(text_file);
        println!("TIME IN CHECK: {:.2}", before.elapsed().as_millis() as f64 / 1000.);
        println!("WORDS IN MISSPELLED: {}", dictionary.misspellings);
        println!("WORDS IN DICTIONARY: {}", dictionary.len());
        println!("WORDS IN TEXT: {}", dictionary.text_words);
    }


    #[test]
    fn test_bible() {
        let dictionary_file: &Path = Path::new("src/data/dictionary.txt");
        let text_file: &Path = Path::new("src/data/bible.txt");


        // if there is a second arg then use that as the dictionary file
        
        // start the timer for load
        let before = Instant::now();
        let mut dictionary = Checker::new(dictionary_file);
        println!("TIME IN LOAD: {:.2}", before.elapsed().as_millis() as f64 / 1000.);
        
        let before = Instant::now();
        dictionary.len();
        println!("TIME IN LENGTH: {:.2}", before.elapsed().as_millis() as f64 / 1000.);

        let before = Instant::now();
        println!("MISSPELLED WORDS");
        println!("-----------------");
        dictionary.check_file(text_file);
        println!("TIME IN CHECK: {:.2}", before.elapsed().as_millis() as f64 / 1000.);
        println!("WORDS IN MISSPELLED: {}", dictionary.misspellings);
        println!("WORDS IN DICTIONARY: {}", dictionary.len());
        println!("WORDS IN TEXT: {}", dictionary.text_words);
    }
}