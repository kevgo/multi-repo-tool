use crate::commands;
use crate::config::Config;
use std::fmt::Display;
use std::process::ExitCode;

#[derive(Debug, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum UserError {
    ApiRequestFailed {
        url: String,
        error: String,
        guidance: String,
    },
    CannotReadDirectory {
        directory: String,
        guidance: String,
    },
    CannotReadPersistenceFile {
        filename: String,
        guidance: String,
    },
    CannotWriteFile {
        filename: String,
        guidance: String,
    },
    InvalidPersistenceFormat {
        filename: String,
        guidance: String,
    },
    NoFoldersToIterate,
    NoNextFolder,
    NotWrapped,
    NothingToAbort,
    NothingToIgnore,
    NothingToRetry,
    SessionAlreadyActive {
        config: Config,
    },
    StepFailed {
        code: u8,
    },
    UnknownApiError {
        url: String,
        code: u16,
        response: String,
    },
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
            UserError::ApiRequestFailed {
                url,
                error,
                guidance,
            } => write!(
                f,
                "cannot read GitHub API:\n- url: {}\n- error: {}\n- guidance: {}",
                url, error, guidance
            ),
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
            UserError::NoFoldersToIterate => write!(f, "all folders have been filtered out"),
            UserError::NotWrapped => {
                write!(
                    f,
                    "please don't call the mrt binary directly, run \"mrt activate | source\" and then call the shell function \"m\""
                )
            }
            UserError::NoNextFolder => write!(f, "no next subfolder"),
            UserError::NothingToAbort => write!(f, "nothing to abort"),
            UserError::NothingToIgnore => write!(f, "nothing to ignore"),
            UserError::NothingToRetry => write!(f, "nothing to retry"),
            #[allow(clippy::print_in_format_impl)]
            UserError::SessionAlreadyActive { config } => {
                commands::status(config);
                println!();
                write!(f, "a session is already active. Please abort this currently running session before starting a new one.")
            }
            UserError::StepFailed { code: _ } => {
                write!(f, "Abort, Retry, Ignore?")
            }
            UserError::UnknownApiError {
                url,
                code,
                response,
            } => write!(
                f,
                "unexpected GitHub API error:\n- url: {}\n- code: {}\n- response: {}",
                url, code, response
            ),
        }
    }
}
