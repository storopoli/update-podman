#[derive(Debug, PartialEq)]
pub enum Status {
    New,
    Old,
}

pub fn get_status(str: &str) -> Status {
    let splits: Vec<String> = str
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();
    let age = &splits[4];
    if age.contains("day") || age.contains("hour") {
        Status::New
    } else {
        Status::Old
    }
}

pub fn get_name(str: &str) -> String {
    let splits: Vec<String> = str
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();
    let name = &splits[0];
    name.clone()
}

pub fn get_image_id(str: &str) -> String {
    let splits: Vec<String> = str
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();
    let name = &splits[2];
    name.clone()
}
