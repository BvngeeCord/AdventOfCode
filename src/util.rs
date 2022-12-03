use std::fs;

pub fn load_from_text_file(filepath: &str, split_pattern: &str) -> Result<Vec<String>, String> {

    if let Ok(file_contents) = fs::read_to_string(filepath) {
        Ok(
            file_contents.split(split_pattern).map(String::from).collect::<Vec<String>>()
        )
    } else {
        Err("Unable to read from file!".to_string())
    }

}
