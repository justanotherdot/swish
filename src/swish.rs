extern crate hex;
extern crate rayon;
extern crate ring;

use rayon::prelude::*;
use ring::digest;
use std::fs;

fn hash_lines(lines: Vec<&str>) -> Vec<String> {
    lines.par_iter().map(|s| to_hex_digest(s)).collect()
}

fn to_hex_digest(s: &str) -> String {
    let digest = digest::digest(&digest::SHA256, s.as_ref());
    hex::encode(digest.as_ref())
}

/// SHA256 a buffer line by line followed by a SHA256 of the joined, ordered result.
pub fn hash_buf(buffer: &str) -> String {
    let lines = buffer.lines().collect();
    let mut shas = hash_lines(lines);
    shas.par_sort();
    let hex_digest = to_hex_digest(&shas.join(""));
    return hex_digest;
}

/// Similar to `hash_buf` but performs the hash on several given files.
pub fn hash_files(files: &Vec<String>) -> Vec<String> {
    files
        .par_iter()
        .map(|file| fs::read_to_string(file))
        .map(|buffer| hash_buf(&buffer.unwrap()))
        .collect()
}

pub fn all_shas_match(shas: &Vec<String>) -> bool {
    let first = &shas[0];
    shas.iter().fold(true, |_, x| x == first)
}
