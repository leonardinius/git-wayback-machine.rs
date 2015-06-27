use std::fmt;
use std::ops::Add;
use std::ops::Range;
use std::path::Path;
use std::process::Command;

use git;

#[derive(Debug, Clone)]
pub struct Entry { name: String, time: String, comment: String, commit: String, }

impl fmt::Display for Entry {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry: {:?} {:?} {:?}: {:?}", self.commit, self.time, self.name, self.comment)
    }
}

impl Entry {
    pub fn new(commit: &str, name: &str, time: &str, comment: &str) -> Entry {
        Entry { commit : commit.to_owned(), name: name.to_owned(), time: time.to_owned(), comment: comment.to_owned() }
    }

    pub fn name(&self) -> &str { &*self.name }
    pub fn time(&self) -> &str { &*self.time }
    pub fn comment(&self) -> &str { &*self.comment }
    pub fn commit(&self) -> &str { &*self.commit }
}

#[derive(Debug, Clone)]
pub struct History<'a> {
    cwd: &'a Path,
    entries_count: Option<i32>,
    page_size: i32,
}

impl<'a> History<'a> {
    const GIT_ONE_LINE_DETAILS : [&'static str;2] = ["log", "--pretty=format:%h|%an|%cr|%s"];

    pub fn new(size: i32, cwd: &Path) -> History {
        History { page_size: size, cwd : cwd, entries_count: Self::__count__(cwd) }
    }

    pub fn entries_count(&self) -> Option<i32> { self.entries_count }

    pub fn page_size(&self) -> i32 { self.page_size }

    pub fn page_count(&self) -> Option<i32> { 
        self.entries_count()
            .map(|e| e + self.page_size())
            .map(|e| e / self.page_size())
    }

    fn __count__(cwd: &Path) -> Option<i32> {
        let args = Self::GIT_ONE_LINE_DETAILS.to_vec();

        git::git_pipe(Command::new("wc").arg("-l"),
                      cwd, args.as_ref())
            .map(|s| s.trim().parse::<i32>().unwrap_or(0))
            .ok()
    }

    pub fn get_page(&self, page: i32) -> Option<Vec<Entry>> {
        self.get_page_data(page)
            .map(|strings| strings.into_iter()
                    .filter_map(|line| Self::make_entry(line))
                    .collect::<Vec<_>>())
    }

    fn make_entry(line: String) -> Option<Entry> {
        let parts: Vec<_> = line.trim().split('|')
            .collect();
        debug!("make_entry: {:?} -> {:?}", line, parts);

        if let [commit, name, time, comment] = &parts[..] {
            Some(Entry::new(commit, name, time, comment))
        } else {
            None
        }
    }

    fn get_page_data(&self, page: i32) -> Option<Vec<String>> {
        let mut args = Self::GIT_ONE_LINE_DETAILS
                        .iter()
                        .map(|e| String::from(*e))
                        .collect::<Vec<String>>();

        let arg1 = format!("--skip={}", self.page_size() * page);
        let arg2 = format!("--max-count={}", self.page_size());

        args.push(arg1);
        args.push(arg2);

        git::git_exec(self.cwd, args.as_ref())
            .ok()
            .map(|s| s.trim().lines()
                 .map(|e| e.trim().to_owned())
                 .filter(|e| !e.is_empty())
                 .collect::<Vec<String>>())
    }
}
