use crate::error::{Error, IOError};

pub fn read_code_file(path: &std::path::PathBuf) -> Result<String, Error> {
    std::fs::read_to_string(path).map_err(|e| {
        let error = match e.kind() {
            std::io::ErrorKind::NotFound => IOError::CodeFileNotFound(e),
            _ => IOError::FailedToReadCodeFile(e),
        };

        Error::IO(error)
    })
}
