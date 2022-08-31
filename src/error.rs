use std::fmt::Display;
use std::process::ExitCode;

#[derive(Debug, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum UserError {
    CannotReadDirectory { directory: String, guidance: String },
    CannotReadPersistenceFile { filename: String, guidance: String },
    CannotWriteFile { filename: String, guidance: String },
    InvalidPersistenceFormat { filename: String, guidance: String },
    NoNextFolder {},
    NothingToAbort {},
    NothingToIgnore {},
    NothingToRetry {},
    StepFailed { code: u8 },
}

impl UserError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            UserError::StepFailed { code } => ExitCode::from(code.to_owned()),
            _ => ExitCode::FAILURE,
        }
    }
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CannotReadDirectory {
                directory,
                guidance,
            } => write!(f, "cannot read directory \"{}\": {}", directory, guidance),
            UserError::CannotReadPersistenceFile { filename, guidance } => write!(
                f,
                "cannot read persistence file \"{}\": {}",
                filename, guidance
            ),
            UserError::CannotWriteFile { filename, guidance } => write!(
                f,
                "cannot write persistence file \"{}\": {}",
                filename, guidance
            ),
            UserError::InvalidPersistenceFormat { filename, guidance } => write!(
                f,
                "persistence file \"{}\" has an invalid format: {}",
                filename, guidance
            ),
            UserError::NoNextFolder {} => write!(f, "no next subfolder"),
            UserError::NothingToAbort {} => write!(f, "nothing to abort"),
            UserError::NothingToIgnore {} => write!(f, "nothing to ignore"),
            UserError::NothingToRetry {} => write!(f, "nothing to retry"),
            UserError::StepFailed { code: _ } => {
                write!(f, "Abort, Retry, Ignore?")
            }
        }
    }
}
