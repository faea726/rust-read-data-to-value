// #[allow(dead_code)]
use std::{collections::HashMap, fs};

pub fn read_data_from_file(filename: &str, delimiter: &str) -> HashMap<String, String> {
    let contents = read_file(filename);

    let data = convert_contents_to_data(contents, delimiter);
    data
}

struct Data {
    key: String,
    value: String,
}
fn convert_contents_to_data(contents: String, delimiter: &str) -> HashMap<String, String> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut data: Vec<Data> = Vec::new();

    for line in lines {
        if !line.contains(delimiter) {
            continue;
        }
        let line_data: Vec<&str> = line.split(delimiter).collect();
        data.push(Data {
            key: line_data[0].to_owned(),
            value: line_data[1].to_owned(),
        })
    }
    let mut data_hash_map = HashMap::new();
    for pair in data {
        data_hash_map.insert(pair.key, pair.value);
    }
    data_hash_map
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("No config file")
}
