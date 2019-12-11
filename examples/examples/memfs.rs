#![allow(clippy::unnecessary_mut_passed)]
#![warn(clippy::unimplemented)]

use polyfuse::{
    op,
    reply::{ReplyAttr, ReplyEntry},
    FileAttr,
};
use polyfuse_examples::prelude::*;
use polyfuse_examples::table::{Node, NodeTable};
use std::{
    collections::HashMap,
    fs::Metadata,
    io,
    os::linux::fs::MetadataExt,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

const TTL: Duration = Duration::from_secs(60 * 60 * 24 * 365); // 1 year.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mountpoint = examples::get_mountpoint()?;
    anyhow::ensure!(mountpoint.is_dir(), "the mountpoint must be a directory");

    let memfs = MemFS::new(&std::fs::metadata(&mountpoint)?);

    polyfuse_tokio::mount(memfs, mountpoint, &[]).await?;

    Ok(())
}

/// An in-memory filesystem.
struct MemFS {
    nodes: NodeTable,
    files: Mutex<HashMap<u64, Arc<MemFSFile>>>,
    ttl_entry: Duration,
    ttl_attr: Duration,
}

impl MemFS {
    /// Create a new `MemFS` mounted on the specified directory.
    pub fn new(metadata: &Metadata) -> Self {
        let mut root_attr = FileAttr::default();
        root_attr.set_uid(metadata.st_uid());
        root_attr.set_gid(metadata.st_gid());

        Self {
            nodes: NodeTable::new(root_attr),
            files: Mutex::default(),
            ttl_attr: TTL,
            ttl_entry: TTL,
        }
    }

    fn make_attr<W: ?Sized>(&self, cx: &Context<'_, W>, mode: u32) -> FileAttr {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let sec = now.as_secs();
        let nsec = now.subsec_nanos();

        let mut attr = FileAttr::default();
        attr.set_nlink(1);
        attr.set_mode(mode);
        attr.set_uid(cx.uid());
        attr.set_gid(cx.gid());
        attr.set_atime(sec, nsec);
        attr.set_mtime(sec, nsec);
        attr.set_ctime(sec, nsec);
        attr
    }

    fn get_file(&self, ino: u64) -> Option<Arc<MemFSFile>> {
        let files = self.files.lock().unwrap();
        files.get(&ino).cloned()
    }

    async fn do_lookup<W: ?Sized>(
        &self,
        cx: &mut Context<'_, W>,
        op: op::Lookup<'_>,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        match self.nodes.lookup(op.parent(), op.name()) {
            Some(node) => {
                let attr = node.attr();
                let mut entry = ReplyEntry::new(attr);
                entry.attr_valid(self.ttl_attr.as_secs(), self.ttl_attr.subsec_nanos());
                entry.entry_valid(self.ttl_entry.as_secs(), self.ttl_entry.subsec_nanos());
                op.reply(cx, entry).await
            }
            None => cx.reply_err(libc::ENOENT).await,
        }
    }

    async fn do_getattr<W: ?Sized>(
        &self,
        cx: &mut Context<'_, W>,
        op: op::Getattr<'_>,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let inode = match self.nodes.get(op.ino()) {
            Some(inode) => inode,
            None => return cx.reply_err(libc::ENOENT).await,
        };
        let mut attr = ReplyAttr::new(inode.attr());
        attr.attr_valid(self.ttl_attr.as_secs(), self.ttl_attr.subsec_nanos());
        op.reply(cx, attr).await
    }

    async fn do_setattr<W: ?Sized>(
        &self,
        cx: &mut Context<'_, W>,
        op: polyfuse::op::Setattr<'_>,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let node = match self.nodes.get(op.ino()) {
            Some(inode) => inode,
            None => return cx.reply_err(libc::ENOENT).await,
        };

        let mut attr = node.attr();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        if let Some(mode) = op.mode() {
            attr.set_mode(mode);
        }
        if let Some(uid) = op.uid() {
            attr.set_uid(uid);
        }
        if let Some(gid) = op.gid() {
            attr.set_gid(gid);
        }
        if let Some(size) = op.size() {
            attr.set_size(size);
        }
        if let Some((s, ns, is_now)) = op.atime() {
            if is_now {
                attr.set_atime(now.as_secs() as u64, now.subsec_nanos());
            } else {
                attr.set_atime(s, ns);
            }
        }
        if let Some((s, ns, is_now)) = op.mtime() {
            if is_now {
                attr.set_mtime(now.as_secs() as u64, now.subsec_nanos());
            } else {
                attr.set_mtime(s, ns);
            }
        }
        if let Some((s, ns)) = op.ctime() {
            attr.set_ctime(s, ns);
        }

        node.set_attr(attr);

        let mut attr = ReplyAttr::new(attr);
        attr.attr_valid(self.ttl_attr.as_secs(), self.ttl_attr.subsec_nanos());
        op.reply(cx, attr).await
    }

    async fn do_read<W: ?Sized>(&self, cx: &mut Context<'_, W>, op: op::Read<'_>) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        match self.get_file(op.ino()) {
            Some(file) => file.read(cx, op).await,
            None => cx.reply_err(libc::ENOENT).await,
        }
    }

    async fn do_write<W: ?Sized, T>(
        &self,
        cx: &mut Context<'_, W>,
        op: op::Write<'_>,
        data: T,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
        T: AsRef<[u8]>,
    {
        match self.get_file(op.ino()) {
            Some(file) => file.write(cx, op, data).await,
            None => cx.reply_err(libc::ENOENT).await,
        }
    }

    async fn do_readdir<W: ?Sized>(
        &self,
        cx: &mut Context<'_, W>,
        op: op::Readdir<'_>,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        match self.nodes.get(op.ino()) {
            Some(node) => match node.read_dir(op.offset()) {
                Ok(entries) => {
                    let mut buf = Vec::with_capacity(op.size() as usize);
                    entries.fill_buffer(&mut buf, op.size());
                    op.reply_vectored(cx, &buf[..]).await
                }
                Err(err) => cx.reply_err(err.as_raw_os_error()).await,
            },
            None => cx.reply_err(libc::ENOENT).await,
        }
    }

    async fn do_mknod<W: ?Sized>(
        &self,
        cx: &mut Context<'_, W>,
        op: op::Mknod<'_>,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        unimplemented!()
    }

    async fn do_mkdir<W: ?Sized>(
        &self,
        cx: &mut Context<'_, W>,
        op: op::Mkdir<'_>,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        unimplemented!()
    }
}

#[async_trait]
impl<T> Filesystem<T> for MemFS
where
    T: AsRef<[u8]>,
{
    async fn call<W: ?Sized>(&self, cx: &mut Context<'_, W>, op: Operation<'_, T>) -> io::Result<()>
    where
        W: AsyncWrite + Send + Unpin + 'async_trait,
        T: Send + 'async_trait,
    {
        match op {
            Operation::Lookup(op) => self.do_lookup(cx, op).await?,
            Operation::Getattr(op) => self.do_getattr(cx, op).await?,
            Operation::Setattr(op) => self.do_setattr(cx, op).await?,
            Operation::Read(op) => self.do_read(cx, op).await?,
            Operation::Write(op, data) => self.do_write(cx, op, data).await?,
            Operation::Readdir(op) => self.do_readdir(cx, op).await?,
            Operation::Mknod(op) => self.do_mknod(cx, op).await?,
            Operation::Mkdir(op) => self.do_mkdir(cx, op).await?,
            Operation::Unlink(op) => unimplemented!(),
            Operation::Rmdir(op) => unimplemented!(),
            _ => (),
        }

        Ok(())
    }
}

struct MemFSFile {
    node: Node,
}

impl MemFSFile {
    async fn read<W: ?Sized>(&self, cx: &mut Context<'_, W>, op: op::Read<'_>) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        unimplemented!()
    }

    async fn write<W: ?Sized, T>(
        &self,
        cx: &mut Context<'_, W>,
        op: op::Write<'_>,
        data: T,
    ) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
        T: AsRef<[u8]>,
    {
        unimplemented!()
    }
}
