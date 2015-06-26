use std::fmt;
use std::path::Path;
use std::process::Command;

use git;

#[derive(Debug, Clone)]
pub struct Entry<'a> { name: &'a str, time: &'a str, comment: &'a str, commit: &'a str, }

impl<'a> fmt::Display for Entry<'a> {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry: {} {} {}: {}", self.commit, self.time, self.name, self.comment)
    }
}

impl<'a> Entry<'a> {
    pub fn new(name: &'a str, time: &'a str, comment: &'a str, commit: &'a str) -> Entry<'a> {
        Entry { name:name, time: time, comment: comment, commit : commit }
    }

    pub fn name(&self) -> &str { self.name }
    pub fn time(&self) -> &str { self.time }
    pub fn comment(&self) -> &str { self.comment }
    pub fn commit(&self) -> &str { self.commit }
}

pub struct History<'a> {
    cwd: &'a Path,
    count: Option<i32>,
}

impl<'a> History<'a> {
    const GIT_ONE_LINE_DETAILS : [&'static str;2] = ["log", "--pretty=format:'%h|%an|%cr|%s'"];

    pub fn new(cwd: &Path) -> History { History{ cwd : cwd, count: Self::__count__(cwd) } }

    fn __count__(cwd: &Path) -> Option<i32> {
        let mut args: Vec<&str> = Self::GIT_ONE_LINE_DETAILS.to_vec();

        git::git_pipe(Command::new("wc").arg("-l"),
                      cwd, &args[ .. ])
            .map(|s| s.trim().parse::<i32>().unwrap_or(0))
            .ok()
    }

    pub fn count(&self) -> Option<i32> { self.count }
}
