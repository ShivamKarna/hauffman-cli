mod error;
mod hauffman;

use error::HauffmanError;
fn main() -> Result<(), HauffmanError> {
    let content = hauffman::read_file("example.txt")?;

    println!("The file content were : {}", content);

    Ok(())
}
