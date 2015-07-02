use log::LogLevel;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::io;
use std::io::{Read, Write};
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
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

pub fn git_exec<S: AsRef<OsStr>>(dir: &Path, args: &[S]) -> Result<String> {
    let git_bin = get_git_bin();
    if log_enabled!(LogLevel::Debug) {
        debug!("Executing `{:?} {:?} {:?}`", git_bin, dir.display(), args.iter().map(|e| e.as_ref().to_str()).collect::<Vec<_>>());
    }

    let output = try!(Command::new(git_bin)
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

pub fn git_pipe<S: AsRef<OsStr>>(pipe: &mut Command, dir: &Path, args: &[S]) -> Result<String> {
    let git_bin = get_git_bin();
    if log_enabled!(LogLevel::Debug) {
        debug!("Pipe `{:?} {:?} {:?}` through {:?}", git_bin, dir.display(), args.iter().map(|e| e.as_ref().to_str()).collect::<Vec<_>>(), pipe);
    }

    let git = try!(Command::new(git_bin)
        .current_dir(dir)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn());

    let mut pipe_process = try!(pipe
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn());

    try!(io::copy(git.stdout.expect("Expected Git stdout").by_ref(),
                  pipe_process.stdin.as_mut().expect("Expected preset Stdio::piped")));

    let output = try!(pipe_process.wait_with_output());

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
    git_exec(dir, &["stash", "-u"])
}

pub fn unstash(dir: &Path) ->  Result<String> {
    git_exec(dir, &["stash", "apply"])
}

pub fn reset(dir: &Path, commit: &str) ->  Result<String> {
    git_exec(dir, &["reset", "--hard", commit])
}

pub fn get_rev_short_sha(dir: &Path, rev: &str) ->  Result<String> {
    git_exec(dir, &["rev-parse", "--short", rev]).map(|s| s.trim().to_string())
}

