mod error;
mod hauffman;

enum Command<'a> {
    Encode { filepath: &'a str },
    Decode { filepath: &'a str },
}

use error::HauffmanError;

impl<'a> Command<'a> {
    fn parse(args: &'a [String]) -> Result<Command<'a>, HauffmanError> {
        match args {
            [_, flag, filepath, ..] => match flag.as_ref() {
                "-en" => Ok(Command::Encode { filepath: filepath }),
                "-den" => Ok(Command::Decode { filepath: filepath }),
                _ => Err(HauffmanError::UnknownFlag),
            },
            _ => Err(HauffmanError::NoArguments),
        }
    }

    pub fn run(self: &Self) -> Result<(), HauffmanError> {
        match self {
            Command::Encode { filepath } => {
                if !filepath.ends_with(".txt") {
                    println!("Error: Only '.txt' files can be Encoded!");
                    return Err(HauffmanError::InvalidHeader);
                }
                println!("Encoding '{}'...", filepath);
                let content = hauffman::read_file(filepath)?;
                let output_path = filepath.replace(".txt", ".en.hauff");

                hauffman::encode_to_file(&content, &output_path)?;
                println!("Success! File encoded to: {}", output_path);
            }

            Command::Decode { filepath } => {
                if !filepath.ends_with(".hauff") {
                    println!("Error: Only '.hauff' files can be Decoded!");
                    return Err(HauffmanError::InvalidHeader);
                }
                println!("Decoding '{}'...", filepath);
                let output_path = filepath.replace(".en.hauff", ".den.hauff");

                hauffman::decode_to_file(filepath, &output_path)?;
                println!("Success! File decoded to: {}", output_path);
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), HauffmanError> {
    let args: Vec<String> = std::env::args().collect();

    let command = Command::parse(&args)?;

    command.run()
}
