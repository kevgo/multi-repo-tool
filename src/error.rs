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
    MissingCommandForExcept,
    MissingCommandForList,
    MissingCommandForOnly,
    MissingStartFolder,
    MissingOrgToClone,
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
                (format!("command \"{}\" not found", command), S(""))
            }
            UserError::ExecutePermissionDenied { command } => {
                (format!("\"{}\" is not executable", command), S(""))
            }
            UserError::InvalidPersistenceFormat { filename, guidance } => (
                format!("persistence file \"{}\" has an invalid format", filename),
                guidance,
            ),
            UserError::MissingCommandForExcept => (
                S("missing condition"),
                S("The \"except\" command filters the currently active directories. It removes those in which the given CLI command returns exit code 0.\n\nYou forgot to tell me the CLI command I should run in each directory. You do it like this:\n\n  m except <cli command>\n\nAs an example, to select all directories that don't contain a Node.js codebase:\n\n  m except test -f package.json"),
            ),
            UserError::MissingCommandForList => (
                S("missing condition"),
                S("The \"list\" command displays all active directories in which the given CLI command returns exit code 0.\nYou forgot to tell me the CLI command I should run in each directory. You do it like this:\n\n  m list <command>\n\nAs an example, to find all codebases that are not Node.js:\n\n  m list test -f package.json"),
            ),
            UserError::MissingCommandForOnly => (
                S("missing condition"),
                S("The \"only\" command filters the set of active directories.\nIt runs the given CLI command in each active directory.\nIf the exit code is 0, it keeps the directory in the list of active directories, otherwise it removes it.\n\nYou forgot to tell me the CLI command I should run in each directory. You do it like this:\n\n  m except <cli command>\n\nAs an example, to find all codebases that are not Node.js:\n\n  m except test -f package.json"),
            ),
            UserError::MissingStartFolder => (
                S("missing start folder"),
                S("The \"walk-from\" command begins a manual iteration starting at the given folder. Usage: m walk-from <folder to start the walk in>")
            ),
            UserError::MissingOrgToClone => (
                S("missing GitHub organization to clone"),
                S("The clone command clones all repositories in a GitHub organization onto your machine.\nYou need to tell me which GitHub organization to clone.\nYou do it like this:\n\n  m clone <GitHub org name>\n\nExample:\n\n  m clone github.com/kevgo"),
            ),
            UserError::NoFoldersToIterate => {
                (S("all folders have been filtered out"), S(""))
            }
            UserError::NotWrapped => (
                S("please don't call the mrt binary directly"),
                S("run \"mrt activate | source\" and then call the shell function \"m\""),
            ),
            UserError::NoNextFolder => (S("no next subfolder"), S("")),
            UserError::NothingToAbort => (S("nothing to abort"), S("")),
            UserError::NothingToIgnore => (S("nothing to ignore"), S("")),
            UserError::NothingToRetry => (S("nothing to retry"), S("")),
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
                S(""),
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
                S("")
            )
        }
    }
}
