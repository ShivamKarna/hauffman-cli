mod error;
mod hauffman;

use error::HauffmanError;
fn main() -> Result<(), HauffmanError> {
    let content = hauffman::read_file("/home/shivamkarn/hauffman/src/example.txt")?;

    let frequencies = hauffman::count_frequencies(&content);

    if let Some(root) = hauffman::build_tree(&frequencies) {
        let codes = hauffman::generate_code(&root);
        println!("=== HUFFMAN CODES FOR FILE ===");
        for (ch, code) in &codes {
            println!("{:?}: {}", ch, code);
        }
    }
    Ok(())
}
