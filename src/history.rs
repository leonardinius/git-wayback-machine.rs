use std::fmt;
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
    pub fn new(commit: &str, name: &str, time: &str, comment: &str) -> Self {
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
    head_rev: String,
    stash: Option<bool>,
    page_size: usize,
}

const GIT_ONE_LINE_DETAILS : [&'static str;2] = ["log", "--pretty=format:%h|%an|%cr|%s"];

impl<'a> History<'a> {
    pub fn new(size: usize, cwd: &'a Path) -> Self {
        let head_rev = git::get_rev_short_sha(cwd, "HEAD")
            .unwrap_or_else(|e| {
                panic!("Failed to init head revision at `{}`: {}", cwd.display(), e);
            });

        History { page_size: size, cwd: cwd, head_rev: head_rev, stash: None }
    }

    pub fn entries_count(&self) -> Option<usize> { Self::__count__(self.cwd) }

    pub fn page_size(&self) -> usize { self.page_size }

    pub fn head_rev(&self) -> &String { &self.head_rev }

    pub fn cwd(&self) -> &str {
        self.cwd.to_str().unwrap_or("?? unknown!")
    }

    pub fn resize(&mut self, page_size: usize) -> &mut Self {
        self.page_size = page_size;

        self
    }

    pub fn page_count(&self) -> Option<usize> {
        self.entries_count()
            // += page_size, so fix the rounding
            .map(|e| (e + self.page_size()) / self.page_size())
    }

    fn __count__(cwd: &Path) -> Option<usize> {
        let args = GIT_ONE_LINE_DETAILS.to_vec();

        git::git_pipe(Command::new("wc").arg("-l"),
                      cwd, args.as_ref())
            .map(|s| s.trim().parse::<usize>().unwrap_or(0))
            .ok()
    }

    pub fn get_page(&self, page: usize) -> Option<Vec<Entry>> {
        self.get_page_data(page)
            .map(|strings| strings.into_iter()
                    .filter_map(|line| Self::make_entry(line))
                    .collect::<Vec<_>>())
    }

    pub fn is_current_commit(&self, entry: &Entry) -> bool {
        git::get_rev_short_sha(self.cwd, "HEAD")
            .map(|sha| sha == entry.commit())
            .unwrap_or(false)
    }

    pub fn reset_to(&mut self, entry: &Entry) -> bool {
        match git::stash(self.cwd) {
            Err(_) => {return false;},
            _ => {;},
        }
        git::reset(self.cwd, entry.commit())
            .map(|_| true)
            .ok()
            .unwrap_or(false)
    }

    fn make_entry(line: String) -> Option<Entry> {
        let parts: Vec<_> = line.trim().split('|')
            .collect();
        debug!("make_entry: {:?} -> {:?}", line, parts);

        assert!(parts.len() == 4, "Git log entry parse error");
        if parts.len() == 4 {
            Some(Entry::new(parts[0], parts[1], parts[2], parts[3]))
        } else {
            None
        }
    }

    fn get_page_data(&self, page: usize) -> Option<Vec<String>> {
        let mut args = GIT_ONE_LINE_DETAILS
                        .iter()
                        .map(|e| String::from(*e))
                        .collect::<Vec<String>>();

        let arg1 = self.head_rev().clone();
        let arg2 = format!("--skip={}", self.page_size() * page);
        let arg3 = format!("--max-count={}", self.page_size());

        args.push(arg1);
        args.push(arg2);
        args.push(arg3);

        git::git_exec(self.cwd, args.as_ref())
            .ok()
            .map(|s| s.trim().lines()
                 .map(|e| e.trim().to_owned())
                 .filter(|e| !e.is_empty())
                 .collect::<Vec<String>>())
    }
}
