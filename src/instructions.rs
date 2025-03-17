use std::fs;

fn load_file_str(path: &str) -> String {
    fs::read_to_string(path).expect("Error reading file. Does it exist?")
}

pub fn load_instructions(path: &str) -> Vec<(i16, i16)> {
    let deserialized_instructions: Vec<(i16, i16)> = serde_json::from_str(&load_file_str(path).to_string()).expect("Failed to parse values");

    return deserialized_instructions;
}
