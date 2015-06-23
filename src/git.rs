use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use std::str;
use std::result;

#[derive(Debug, PartialEq)]
enum GitCommandError {
    Unknown,
    ExecCommand(io::Error),
    ExitCode(i32, str)
}

impl Error for GitCommandError {
    pub fn description(&self) -> &str {
        match *self {
            GitCommandError::Unknown => "Unknown error",
            GitCommandError::ExecCommand(ioe) => {
                format!("Failed to execute GIT command ({})", ioe.description())
            },
            GitCommandError::ExitCode(code, out) => {
                format!("Exit code {} (Out: {})", code, out)
            },
        }
    }

    pub fn cause(&self) -> Option<&Error> { 
        match *self {
            GitCommandError::ExecCommand(ref ioe) => Some(ioe),
            _ => None,
        }
    }
}

impl From<io::Error> for GitCommandError {
    pub fn from(cause: io::Error) -> Self {
        GitCommandError::ExecCommand(cause)
    }
}

impl fmt::Display for GitCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

fn get_git_bin() -> PathBuf {
    env::var_os("GIT_BIN_PATH").map(PathBuf::from).unwrap_or("").join("git")
}

fn get_current_dir() -> Path {
    env::current_dir().as_path()
}

pub type Result<T> = result::Result<T, GitCommandError>;

fn exec_git(dir: &Path, args: &[str]) -> Result<String> {
    let git = get_git_bin();
    debug!("Executing {} {}", dir, args);

    let output = try!(process::Command::new(git)
        .current_dir(dir)
        .args(args)
        .output());
    debug!("Output: {}", output);

    if output.status.success() {
        Result::Ok(String::from_utf8_lossy(&output.stdout))
    } else {
        let out = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);
        let code = output.status.code().unwrap_or(-1);
        Result::Err(GitCommandError::ExitCode(code, out + err))
    }
}

pub fn stash(dir: &Path) ->  Result<String> {
    exec_git(dir, &["stash", "-u"])
}

pub fn reset(dir: &Path, commit: &str) ->  Result<String> {
    exec_git(dir, &["reset", "--hard", commit])
}
