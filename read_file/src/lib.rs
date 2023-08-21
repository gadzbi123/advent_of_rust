use std::fs::read_to_string;

pub fn read_file(file_name: &str) -> Vec<String> {
    read_to_string(file_name)
        .expect("should read a file")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}
