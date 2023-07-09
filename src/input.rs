mod input {
    pub fn input_lines(year: u32, day: u8) -> Vec<String>;
}

fn input_source(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
