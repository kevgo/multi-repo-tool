use crate::runtime::Step;
use std::fmt::Display;
use std::process::ExitCode;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum UserError {
    CannotDeletePersistenceFile { filename: String, guidance: String },
    CannotReadPersistenceFile { filename: String, guidance: String },
    CannotWritePersistenceFile { filename: String, guidance: String },
    InvalidPersistenceFormat { filename: String, guidance: String },
    StepFailed { step: Step, exit_code: u8 },
}

impl UserError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            UserError::StepFailed { step: _, exit_code } => ExitCode::from(exit_code.to_owned()),
            _ => ExitCode::FAILURE,
        }
    }
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CannotDeletePersistenceFile { filename, guidance } => {
                write!(
                    f,
                    "cannot delete persistence file \"{}\": {}",
                    filename, guidance
                )
            }
            UserError::CannotReadPersistenceFile { filename, guidance } => write!(
                f,
                "cannot read persistence file \"{}\": {}",
                filename, guidance
            ),
            UserError::CannotWritePersistenceFile { filename, guidance } => {
                write!(
                    f,
                    "cannot write persistence file \"{}\": {}",
                    filename, guidance
                )
            }
            UserError::InvalidPersistenceFormat { filename, guidance } => {
                write!(
                    f,
                    "persistence file \"{}\" has an invalid format: {}",
                    filename, guidance
                )
            }
            UserError::StepFailed { step, exit_code: _ } => {
                write!(f, "step {} failed\n\nAbort, Retry, Ignore?", step)
            }
        }
    }
}
