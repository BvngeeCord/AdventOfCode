use std::fs;

pub fn load_from_text_file(year: u16, day: u16) -> Result<String, String> {

    if let Ok(file_contents) = fs::read_to_string(format!("./src/aoc_{}/day_{}/input.txt", year, day)) {
        Ok(file_contents)
    } else {
        Err("Unable to read from file!".to_string())
    }

}
