use std::fmt;
use std::io;

//* Erros enum
#[derive(Debug)]
pub enum HauffmanError {
    Io(io::Error),
    // user provided empty file to encode
    EmptyFile,
    //TODO: Provided file doesn't has .hauff or .txt ( i will decide later which extension to keep in the code)
    InvalidHeader,
    // the specific string/word/letter was not valid , that could be encoded.
    CorruptedData,
}

impl fmt::Display for HauffmanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HauffmanError::Io(err) => write!(f, "I/O Error! : {}", err),
            HauffmanError::EmptyFile => {
                write!(f, "Empty file was given !, Cannot compress empty file.")
            }
            // ! Reminder to set either both or one of the file extension.. ".txt" or ".hauff"
            HauffmanError::InvalidHeader => write!(
                f,
                "File is not valid, i.e. Should be either .hauff or .txt file"
            ),

            HauffmanError::CorruptedData => {
                write!(f, "Failed to decompress: The byte is corrupted.")
            }
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
