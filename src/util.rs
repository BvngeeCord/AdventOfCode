use std::fs;

pub fn load_from_text_file(filepath: &str) -> Result<String, String> {

    if let Ok(file_contents) = fs::read_to_string(filepath) {
        Ok(file_contents)
    } else {
        Err("Unable to read from file!".to_string())
    }

}
