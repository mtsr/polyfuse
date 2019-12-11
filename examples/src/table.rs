#![warn(clippy::unimplemented)]

use polyfuse::{DirEntry, FileAttr};
use std::ffi::OsStr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoEntry,
}

impl Error {
    pub fn as_raw_os_error(&self) -> i32 {
        match self {
            Self::NoEntry => libc::ENOENT,
        }
    }
}

#[derive(Debug)]
pub struct NodeTable {
    _p: (),
}

impl NodeTable {
    pub fn new(root_attr: FileAttr) -> Self {
        Self { _p: () }
    }

    pub fn lookup(&self, parent: u64, name: &OsStr) -> Option<Node> {
        None
    }

    pub fn get(&self, ino: u64) -> Option<Node> {
        None
    }
}

#[derive(Debug)]
pub struct Node {
    _p: (),
}

impl Node {
    pub fn attr(&self) -> FileAttr {
        unimplemented!()
    }

    pub fn set_attr(&self, attr: FileAttr) {
        unimplemented!()
    }

    pub fn new_file(&self, name: &OsStr, attr: FileAttr) -> Result<Node> {
        unimplemented!()
    }

    pub fn new_dir(&self, name: &OsStr, attr: FileAttr) -> Result<Node> {
        unimplemented!()
    }

    pub fn new_link(&self, name: &OsStr, ino: u64) -> Result<Node> {
        unimplemented!()
    }

    pub fn new_symlink(&self, name: &OsStr, link: &OsStr, attr: FileAttr) -> Result<Node> {
        unimplemented!()
    }

    pub fn remove(&self) {
        unimplemented!()
    }

    pub fn read_dir(&self, offset: u64) -> Result<ReadDir> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ReadDir {
    _p: (),
}

impl ReadDir {
    pub fn fill_buffer(&self, buf: &mut Vec<&[u8]>, bufsize: u32) {
        unimplemented!()
    }
}
