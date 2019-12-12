#![warn(clippy::unimplemented)]

use polyfuse::FileAttr;
use std::{
    ffi::OsStr,
    sync::{Arc, Weak},
};

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

/// In-memory inode table.
///
/// It manages the hierarchical structure of inodes in a filesystem.
#[derive(Debug)]
pub struct NodeTable {
    table: Arc<imp::Table>,
    root: Weak<imp::Node>,
}

impl NodeTable {
    /// Create a new `NodeTable`.
    ///
    /// The constructor takes an attribute of the root directory and uses its value.
    /// At this time, some properties, such as `ino` are replaced with the appropriate
    /// values.
    pub fn new(root_attr: FileAttr) -> Self {
        let table = Arc::new(imp::Table::new(root_attr));
        let root = table.root();
        Self { table, root }
    }

    /// Return a handle corresponding to the root node.
    pub fn root(&self) -> Node {
        Node {
            node: self.table.root(),
            table: Arc::downgrade(&self.table),
        }
    }

    /// Find a node by the inode number and return its handle.
    pub fn get(&self, ino: u64) -> Option<Node> {
        let node = self.table.get(ino)?;
        Some(Node {
            node,
            table: Arc::downgrade(&self.table),
        })
    }

    /// Lookup a node by the number of parent node and name.
    ///
    /// If the node is found, the lookup count is incremented.
    pub fn lookup(&self, parent: u64, name: &OsStr) -> Option<Node> {
        unimplemented!()
    }

    pub fn forget(&self, ino: u64, nlookup: u64) {
        unimplemented!()
    }
}

/// The node handle.
#[derive(Debug)]
pub struct Node {
    node: Weak<imp::Node>,
    table: Weak<imp::Table>,
}

impl Node {
    #[inline]
    fn upgrade_node(&self) -> Arc<imp::Node> {
        self.node.upgrade().expect("the node is died")
    }

    #[inline]
    fn upgrade_table(&self) -> Arc<imp::Table> {
        self.table.upgrade().expect("the table is died")
    }

    pub fn attr(&self) -> FileAttr {
        self.upgrade_node().attr()
    }

    pub fn set_attr(&self, attr: FileAttr) {
        self.upgrade_node().set_attr(attr)
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

mod imp {
    use crate::shared_map::SharedMap;
    use indexmap::IndexMap;
    use polyfuse::{DirEntry, FileAttr};
    use std::{
        collections::HashMap,
        ffi::OsString,
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc, Mutex, Weak,
        },
    };

    #[derive(Debug)]
    pub struct Table {
        nodes: SharedMap<u64, Node>,
        root: Arc<Node>,
        next_ino: AtomicU64,
    }

    impl Table {
        pub fn new(mut root_attr: FileAttr) -> Self {
            root_attr.set_ino(1);
            root_attr.set_mode((root_attr.mode() & !libc::S_IFMT) & libc::S_IFDIR);

            Self {
                nodes: SharedMap::new(),
                root: Arc::new(Node {
                    nodeid: 1,
                    attr: Mutex::new(root_attr),
                    nlookup: AtomicU64::new(0),
                    parent: Mutex::new(None),
                    kind: NodeKind::Dir(DirNode {
                        children: Mutex::default(),
                        dirents: [
                            Arc::new(DirEntry::dir(".", 1, 1)),
                            Arc::new(DirEntry::dir("..", 1, 2)),
                        ],
                    }),
                }),
                next_ino: AtomicU64::new(2), // ino=1 is used by the root node.
            }
        }

        pub fn root(&self) -> Weak<Node> {
            Arc::downgrade(&self.root)
        }

        pub fn get(&self, ino: u64) -> Option<Weak<Node>> {
            match ino {
                1 => Some(self.root()),
                ino => unimplemented!(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Node {
        nodeid: u64,
        attr: Mutex<FileAttr>,
        nlookup: AtomicU64,
        parent: Mutex<Option<Weak<Node>>>,
        kind: NodeKind,
    }

    #[derive(Debug)]
    enum NodeKind {
        File,
        Dir(DirNode),
    }

    impl Node {
        pub fn attr(&self) -> FileAttr {
            *self.attr.lock().unwrap()
        }

        pub fn set_attr(&self, attr: FileAttr) {
            *self.attr.lock().unwrap() = attr;
        }
    }

    #[derive(Debug)]
    struct DirNode {
        children: Mutex<IndexMap<OsString, DirNodeLeaf>>,
        dirents: [Arc<DirEntry>; 2],
    }

    #[derive(Debug)]
    struct DirNodeLeaf {
        node: Weak<Node>,
        dirent: Arc<DirEntry>,
    }
}
