use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::process;
use std::result;

#[allow(dead_code)]
#[derive(Debug)]
enum GitCommandError {
    Unknown,
    ExecCommand(io::Error),
    ExitCode(i32, String)
}

impl Error for GitCommandError {
    fn description(&self) -> &str {
        match *self {
            GitCommandError::Unknown => "Unknown error",
            GitCommandError::ExecCommand(ref ioe) => ioe.description(),
            GitCommandError::ExitCode(_,_) => "Exit code indicated error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GitCommandError::ExecCommand(ref ioe) => Some(ioe),
            _ => None,
        }
    }
}

impl From<io::Error> for GitCommandError {
    fn from(cause: io::Error) -> Self {
        GitCommandError::ExecCommand(cause)
    }
}

impl fmt::Display for GitCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

fn get_git_bin() -> PathBuf {
    env::var_os("GIT_BIN_PATH")
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from(""))

        .join("git")
}

pub type Result<T> = result::Result<T, GitCommandError>;

pub fn exec_git(dir: &Path, args: &[&str]) -> Result<String> {
    let git = get_git_bin();
    debug!("Executing {:?} {:?} {:?}", git.display(), dir, args);

    let output = try!(process::Command::new(git)
        .current_dir(dir)
        .args(args)
        .output());

    if output.status.success() {
        let txt = String::from_utf8_lossy(&output.stdout);
        debug!("Success: out={:?}", txt);
        Ok(String::from(&*txt))
    } else {
        let out = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);
        let code = output.status.code().unwrap_or(-1);
        debug!("Error: code={}, stdout={:?}, stderr={:?}", code, out, err);
        Err(GitCommandError::ExitCode(code, String::from(&*out).add(&*err)))
    }
}

pub fn stash(dir: &Path) ->  Result<String> {
    exec_git(dir, &["stash", "-u"])
}

pub fn reset(dir: &Path, commit: &str) ->  Result<String> {
    exec_git(dir, &["reset", "--hard", commit])
}
