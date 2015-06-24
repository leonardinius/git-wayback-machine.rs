use std::convert::From;
use std::fmt;
use std::path::Path;

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
}

const GIT_ONE_LINE_DETAILS : &'static str = "log --pretty=format:'%h|%an|%cr|%s'";

impl<'a> History<'a> {

    pub fn new(cwd: &Path) -> History { History{ cwd : cwd } }

    pub fn len(&self) -> i32 {
        let mut args: Vec<&str> = vec![GIT_ONE_LINE_DETAILS];
        args.push(" | wc -l");

        git::exec_git(self.cwd, &args[ .. ]);

        -1
    }
}
