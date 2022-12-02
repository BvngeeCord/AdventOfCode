use std::io::Error;
use std::fs;

pub fn load_from_text_file(filepath: &str, split_pattern: &str) -> Result<Vec<String>, Error> {
    let file_contents: String = fs::read_to_string(filepath)?;
    Ok(
        file_contents.split(split_pattern).map(String::from).collect::<Vec<String>>()

    )
}
