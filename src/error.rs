use crate::config::Config;
use big_s::S;
use std::process::ExitCode;

#[derive(Debug, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum UserError {
    ApiRequestFailed {
        url: String,
        error: String,
        guidance: String,
    },
    CannotChangeIntoDirectory {
        dir: String,
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
    CommandNotFound {
        command: String,
    },
    ExecutePermissionDenied {
        command: String,
    },
    InvalidPersistenceFormat {
        filename: String,
        guidance: String,
    },
    MissingStartFolder,
    NoFoldersToIterate,
    NoNextFolder,
    NotWrapped,
    NothingToAbort,
    NothingToIgnore,
    NothingToRetry,
    OtherExecutionError {
        command: String,
        guidance: String,
    },
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
    WrongCliArguments {
        message: String,
    },
}

impl UserError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            UserError::StepFailed { code } => ExitCode::from(code.to_owned()),
            _ => ExitCode::FAILURE,
        }
    }

    /// provides the error message and guidance for this error
    pub fn messages(self) -> (String, String) {
        match self {
            UserError::ApiRequestFailed {
                url,
                error,
                guidance,
            } => (
                format!("cannot read GitHub API: {}", error),
                format!("url: {}\nguidance: {}", url, guidance),
            ),
            UserError::CannotChangeIntoDirectory { dir, guidance } => (
                format!("cannot change into the \"{}\" directory", dir),
                guidance,
            ),
            UserError::CannotReadDirectory {
                directory,
                guidance,
            } => (format!("cannot read directory \"{}\"", directory), guidance),
            UserError::CannotReadPersistenceFile { filename, guidance } => (
                format!("cannot read persistence file \"{}\"", filename),
                guidance,
            ),
            UserError::CannotWriteFile { filename, guidance } => (
                format!("cannot write persistence file \"{}\"", filename),
                guidance,
            ),
            UserError::CommandNotFound { command } => {
                (format!("command \"{}\" not found", command), String::new())
            }
            UserError::ExecutePermissionDenied { command } => {
                (format!("\"{}\" is not executable", command), String::new())
            }
            UserError::InvalidPersistenceFormat { filename, guidance } => (
                format!("persistence file \"{}\" has an invalid format", filename),
                guidance,
            ),
            UserError::MissingStartFolder => (
                "missing start folder".into(),
                "The \"walk-from\" command begins a manual iteration starting at the given folder. Usage: m walk-from <folder to start the walk in>".into()
            ),
            UserError::NoFoldersToIterate => {
                (S("all folders have been filtered out"), String::new())
            }
            UserError::NotWrapped => (
                S("please don't call the mrt binary directly"),
                S("run \"mrt activate | source\" and then call the shell function \"m\""),
            ),
            UserError::NoNextFolder => (S("no next subfolder"), String::new()),
            UserError::NothingToAbort => (S("nothing to abort"), String::new()),
            UserError::NothingToIgnore => (S("nothing to ignore"), String::new()),
            UserError::NothingToRetry => (S("nothing to retry"), String::new()),
            UserError::OtherExecutionError { command, guidance } => (
                format!("unknown error while trying to execute \"{}\"", command),
                guidance,
            ),
            UserError::SessionAlreadyActive { config } => (
                S("a session is already active. Please abort this currently running session before starting a new one."),
                config.to_string()
            ),
            UserError::StepFailed { code: _ } => (
                S("Abort, Retry, Ignore?"),
                String::new(),
             ),
            UserError::UnknownApiError {
                url,
                code,
                response,
            } => (
                format!("unexpected GitHub API error: {}", code),
                format!("url: {}responseh: {}", url, response),
            ),
            UserError::WrongCliArguments { message } => (
                message,
                String::new()
            )
        }
    }
}
