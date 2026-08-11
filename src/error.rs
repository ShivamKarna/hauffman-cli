use std::fmt;
use std::io;

//* Erros enum
#[derive(Debug)]
pub enum HauffmanError {
    Io(io::Error),
    // user provided empty file to encode
    EmptyFile,
    InvalidHeader,
    // the specific string/word/letter was not valid , that could be encoded.
    CorruptedData,
    // InvalidFlag,
    UnknownFlag, // added for flag checking
    NoArguments, // user entered 0 arguments, means like they just ran ,"cargo run"
}

impl fmt::Display for HauffmanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HauffmanError::Io(err) => write!(f, "I/O Error! : {}", err),
            HauffmanError::EmptyFile => {
                write!(f, "Empty file was given !, Cannot compress empty file.")
            }
            HauffmanError::InvalidHeader => write!(
                f,
                "File is not valid, i.e. Should be either .hauff or .txt file"
            ),

            HauffmanError::CorruptedData => {
                write!(f, "Failed to decompress: The byte is corrupted.")
            }
            HauffmanError::UnknownFlag => write!(
                f,
                "Unknown flag used for encoding/decoding. Use '-en' or '-den'."
            ),
            HauffmanError::NoArguments => write!(
                f,
                "Hauffman CLI ,Missing arguments!\nUsage:\n  cargo run -- -en <filename.txt>\n  cargo run -- -den <filename.en.hauff>"
            ),
        }
    }
}

// Marked as standard rust errors
impl std::error::Error for HauffmanError {}

// Impl From<io::Error for HauffmanError{}
impl From<io::Error> for HauffmanError {
    fn from(err: io::Error) -> Self {
        HauffmanError::Io(err)
    }
}
