extern crate hex;
extern crate rayon;
extern crate ring;

use rayon::prelude::*;
use ring::digest;
use std::fs;

pub struct Swish {
    files: Vec<String>,
    shas: Option<Vec<String>>,
}

impl Swish {
    pub fn new(files: Vec<String>) -> Self {
        Swish { files, shas: None }
    }

    /// Similar to `hash_buf` but performs the hash on several given files.
    pub fn hash(mut self) -> Self {
        let shas = self
            .files
            .par_iter()
            .map(|file| fs::read_to_string(file))
            .map(|buffer| Swish::hash_buf(&buffer.unwrap()))
            .collect();
        self.shas = Some(shas);
        self
    }

    pub fn all_match(&self) -> bool {
        match &self.shas {
            Some(shas) => {
                let first = &shas[0];
                shas.iter().fold(true, |_, x| x == first)
            }
            None => false,
        }
    }

    fn hash_lines(lines: Vec<&str>) -> Vec<String> {
        lines.par_iter().map(|s| Swish::to_hex_digest(s)).collect()
    }

    fn to_hex_digest(s: &str) -> String {
        let digest = digest::digest(&digest::SHA256, s.as_ref());
        hex::encode(digest.as_ref())
    }

    /// SHA256 a buffer line by line followed by a SHA256 of the joined, ordered result.
    fn hash_buf(buffer: &str) -> String {
        let lines = buffer.lines().collect();
        let mut shas = Swish::hash_lines(lines);
        shas.par_sort();
        let hex_digest = Swish::to_hex_digest(&shas.join(""));
        return hex_digest;
    }
}
