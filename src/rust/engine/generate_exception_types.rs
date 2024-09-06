use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Deserialize)]
struct ExceptionType {
    name: String,
    extends: String,
}

pub fn generate_exceptions() {
    let path = env::current_dir()
        .unwrap()
        .join("..")
        .join("..")
        .join("interop")
        .join("exceptions.jsonl");

    let file = File::open(path)
        .expect(format!("Could not open file for path: {}", path.as_path()).as_str());
    let reader = BufReader::new(file);

    let exception_types = reader
        .lines()
        .into_iter()
        .enumerate()
        .map(|line_res| {
            let line = line_res.1.unwrap();
            let exception_type: ExceptionType = serde_json::from_str(line.as_str()).unwrap();
            exception_type
        })
        .collect::<Vec<ExceptionType>>()
        .sort_by_key(|exception_type| &exception_type.name);


}
