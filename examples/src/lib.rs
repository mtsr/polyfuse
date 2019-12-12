#![allow(clippy::unnecessary_mut_passed)]
#![deny(clippy::unimplemented)]

mod shared_map;
pub mod table;

pub mod prelude {
    pub use anyhow::{anyhow, ensure};
    pub use futures::{
        future::{Future, FutureExt},
        io::AsyncWrite,
    };
    pub use polyfuse::{async_trait, op::Operation, Context, Filesystem};
    pub use std::{
        ffi::{OsStr, OsString},
        os::unix::ffi::OsStrExt,
        path::{Path, PathBuf},
    };

    pub use crate as examples;
}

pub use crate::shared_map::SharedMap;

pub fn get_mountpoint() -> anyhow::Result<std::path::PathBuf> {
    std::env::args()
        .nth(1)
        .map(Into::into)
        .ok_or_else(|| anyhow::anyhow!("missing mountpoint"))
}
