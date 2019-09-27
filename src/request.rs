//! Requests from the kernel.

use fuse_async_abi::{
    AccessIn, //
    BmapIn,
    CreateIn,
    FlushIn,
    ForgetIn,
    FsyncIn,
    GetattrIn,
    GetxattrIn,
    InHeader,
    InitIn,
    LinkIn,
    LkIn,
    MkdirIn,
    MknodIn,
    Opcode,
    OpenIn,
    ReadIn,
    ReleaseIn,
    RenameIn,
    SetattrIn,
    SetxattrIn,
    WriteIn,
};
use std::{ffi::OsStr, io, mem, os::unix::ffi::OsStrExt};

#[derive(Debug)]
pub enum Arg<'a> {
    Init(&'a InitIn),
    Destroy,
    Lookup {
        name: &'a OsStr,
    },
    Forget(&'a ForgetIn),
    Getattr(&'a GetattrIn),
    Setattr(&'a SetattrIn),
    Readlink,
    Symlink {
        name: &'a OsStr,
        link: &'a OsStr,
    },
    Mknod {
        arg: &'a MknodIn,
        name: &'a OsStr,
    },
    Mkdir {
        arg: &'a MkdirIn,
        name: &'a OsStr,
    },
    Unlink {
        name: &'a OsStr,
    },
    Rmdir {
        name: &'a OsStr,
    },
    Rename {
        arg: &'a RenameIn,
        name: &'a OsStr,
        newname: &'a OsStr,
    },
    Link {
        arg: &'a LinkIn,
        newname: &'a OsStr,
    },
    Open(&'a OpenIn),
    Read(&'a ReadIn),
    Write {
        arg: &'a WriteIn,
        data: &'a [u8],
    },
    Release(&'a ReleaseIn),
    Statfs,
    Fsync(&'a FsyncIn),
    Setxattr {
        arg: &'a SetxattrIn,
        name: &'a OsStr,
        value: &'a [u8],
    },
    Getxattr {
        arg: &'a GetxattrIn,
        name: &'a OsStr,
    },
    Listxattr {
        arg: &'a GetxattrIn,
    },
    Removexattr {
        name: &'a OsStr,
    },
    Flush(&'a FlushIn),
    Opendir(&'a OpenIn),
    Readdir(&'a ReadIn),
    Releasedir(&'a ReleaseIn),
    Fsyncdir(&'a FsyncIn),
    Getlk(&'a LkIn),
    Setlk(&'a LkIn),
    Setlkw(&'a LkIn),
    Access(&'a AccessIn),
    Create(&'a CreateIn),
    Bmap(&'a BmapIn),
    // Interrupt,
    // Ioctl,
    // Poll,
    // NotifyReply,
    // BatchForget,
    // Fallocate,
    // Readdirplus,
    // Rename2,
    // Lseek,
    // CopyFileRange,
    Unknown {
        opcode: Opcode,
        payload: &'a [u8],
    },
}

pub fn parse<'a>(buf: &'a [u8]) -> io::Result<(&'a InHeader, Arg<'a>)> {
    let (header, payload) = parse_header(buf)?;
    if buf.len() < header.len() as usize {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "received data is too short",
        ));
    }

    let arg = parse_arg(header, payload)?;

    Ok((header, arg))
}

pub fn parse_arg<'a>(header: &InHeader, payload: &'a [u8]) -> io::Result<Arg<'a>> {
    let arg = match header.opcode() {
        Opcode::FUSE_INIT => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Init(arg)
        }
        Opcode::FUSE_DESTROY => {
            debug_assert!(payload.is_empty());
            Arg::Destroy
        }
        Opcode::FUSE_LOOKUP => {
            let (name, remains) = fetch_str(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Lookup { name }
        }
        Opcode::FUSE_FORGET => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Forget(arg)
        }
        Opcode::FUSE_GETATTR => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Getattr(arg)
        }
        Opcode::FUSE_SETATTR => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Setattr(arg)
        }
        Opcode::FUSE_READLINK => {
            debug_assert!(payload.is_empty());
            Arg::Readlink
        }
        Opcode::FUSE_SYMLINK => {
            let (name, remains) = fetch_str(payload)?;
            let (link, _remains) = fetch_str(remains)?;
            debug_assert!(remains.is_empty());
            Arg::Symlink { name, link }
        }
        Opcode::FUSE_MKNOD => {
            let (arg, remains) = fetch(payload)?;
            let (name, remains) = fetch_str(remains)?;
            debug_assert!(remains.is_empty());
            Arg::Mknod { arg, name }
        }
        Opcode::FUSE_MKDIR => {
            let (arg, remains) = fetch(payload)?;
            let (name, remains) = fetch_str(remains)?;
            debug_assert!(remains.is_empty());
            Arg::Mkdir { arg, name }
        }
        Opcode::FUSE_UNLINK => {
            let (name, remains) = fetch_str(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Unlink { name }
        }
        Opcode::FUSE_RMDIR => {
            let (name, remains) = fetch_str(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Rmdir { name }
        }
        Opcode::FUSE_RENAME => {
            let (arg, remains) = fetch(payload)?;
            let (name, remains) = fetch_str(remains)?;
            let (newname, _remains) = fetch_str(remains)?;
            debug_assert!(remains.is_empty());
            Arg::Rename { arg, name, newname }
        }
        Opcode::FUSE_LINK => {
            let (arg, remains) = fetch(payload)?;
            let (newname, _remains) = fetch_str(remains)?;
            debug_assert!(remains.is_empty());
            Arg::Link { arg, newname }
        }
        Opcode::FUSE_OPEN => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Open(arg)
        }
        Opcode::FUSE_READ => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Read(arg)
        }
        Opcode::FUSE_WRITE => {
            let (arg, data) = fetch(payload)?;
            Arg::Write { arg, data }
        }
        Opcode::FUSE_RELEASE => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Release(arg)
        }
        Opcode::FUSE_STATFS => {
            debug_assert!(payload.is_empty());
            Arg::Statfs
        }
        Opcode::FUSE_FSYNC => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Fsync(arg)
        }
        Opcode::FUSE_SETXATTR => {
            let (arg, remains) = fetch(payload)?;
            let (name, value) = fetch_str(remains)?;
            Arg::Setxattr { arg, name, value }
        }
        Opcode::FUSE_GETXATTR => {
            let (arg, remains) = fetch(payload)?;
            let (name, remains) = fetch_str(remains)?;
            debug_assert!(remains.is_empty());
            Arg::Getxattr { arg, name }
        }
        Opcode::FUSE_LISTXATTR => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Listxattr { arg }
        }
        Opcode::FUSE_REMOVEXATTR => {
            let (name, remains) = fetch_str(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Removexattr { name }
        }
        Opcode::FUSE_FLUSH => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Flush(arg)
        }
        Opcode::FUSE_OPENDIR => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Opendir(arg)
        }
        Opcode::FUSE_READDIR => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Readdir(arg)
        }
        Opcode::FUSE_RELEASEDIR => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Releasedir(arg)
        }
        Opcode::FUSE_FSYNCDIR => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Fsyncdir(arg)
        }
        Opcode::FUSE_GETLK => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Getlk(arg)
        }
        Opcode::FUSE_SETLK => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Setlk(arg)
        }
        Opcode::FUSE_SETLKW => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Setlkw(arg)
        }
        Opcode::FUSE_ACCESS => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Access(arg)
        }
        Opcode::FUSE_CREATE => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Create(arg)
        }
        Opcode::FUSE_BMAP => {
            let (arg, remains) = fetch(payload)?;
            debug_assert!(remains.is_empty());
            Arg::Bmap(arg)
        }
        // Opcode::FUSE_INTERRUPT => unimplemented!(),
        // Opcode::FUSE_IOCTL => unimplemented!(),
        // Opcode::FUSE_POLL => unimplemented!(),
        // Opcode::FUSE_NOTIFY_REPLY => unimplemented!(),
        // Opcode::FUSE_BATCH_FORGET => unimplemented!(),
        // Opcode::FUSE_FALLOCATE => unimplemented!(),
        // Opcode::FUSE_READDIRPLUS => unimplemented!(),
        // Opcode::FUSE_RENAME2 => unimplemented!(),
        // Opcode::FUSE_LSEEK => unimplemented!(),
        // Opcode::FUSE_COPY_FILE_RANGE => unimplemented!(),
        opcode => Arg::Unknown { opcode, payload },
    };

    Ok(arg)
}

fn parse_header<'a>(buf: &'a [u8]) -> io::Result<(&'a InHeader, &'a [u8])> {
    const IN_HEADER_SIZE: usize = mem::size_of::<InHeader>();

    if buf.len() < IN_HEADER_SIZE {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "in_header"));
    }
    let (header, remains) = buf.split_at(IN_HEADER_SIZE);

    let header = unsafe { &*(header.as_ptr() as *const InHeader) };

    Ok((header, remains))
}

fn fetch<'a, T>(buf: &'a [u8]) -> io::Result<(&'a T, &'a [u8])> {
    if buf.len() < mem::size_of::<T>() {
        return Err(io::ErrorKind::InvalidData.into());
    }
    let (data, remains) = buf.split_at(mem::size_of::<T>());
    Ok((unsafe { &*(data.as_ptr() as *const T) }, remains))
}

fn fetch_str<'a>(buf: &'a [u8]) -> io::Result<(&'a OsStr, &'a [u8])> {
    let pos = buf.iter().position(|&b| b == b'\0');
    let (s, remains) = if let Some(pos) = pos {
        let (s, remains) = buf.split_at(pos);
        let remains = &remains[1..]; // skip '\0'
        (s, remains)
    } else {
        (buf, &[] as &[u8])
    };
    Ok((OsStr::from_bytes(s), remains))
}
