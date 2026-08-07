use crate::error::HauffmanError;

pub fn read_file(path: &str) -> Result<String, HauffmanError> {
    let content = std::fs::read_to_string(path)?;

    if content.is_empty() {
        return Err(HauffmanError::EmptyFile);
    }

    Ok(content)
}
