//! Filesystem abstraction.

use crate::{
    async_trait,
    common::Forget,
    op::Operation,
    reply::ReplyWriter,
    request::RequestHeader,
    session::{Interrupt, Session},
};
use std::{fmt, future::Future, io, pin::Pin};

/// Contextural information about an incoming request.
pub struct Context<'a> {
    header: &'a RequestHeader,
    session: &'a Session,
}

impl fmt::Debug for Context<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Context").finish()
    }
}

impl<'a> Context<'a> {
    pub(crate) fn new(header: &'a RequestHeader, session: &'a Session) -> Self {
        Self { header, session }
    }

    /// Return the user ID of the calling process.
    pub fn uid(&self) -> u32 {
        self.header.uid()
    }

    /// Return the group ID of the calling process.
    pub fn gid(&self) -> u32 {
        self.header.gid()
    }

    /// Return the process ID of the calling process.
    pub fn pid(&self) -> u32 {
        self.header.pid()
    }

    /// Register the request with the sesssion and get a signal
    /// that will be notified when the request is canceld by the kernel.
    pub async fn on_interrupt(&mut self) -> Interrupt {
        self.session.enable_interrupt(self.header.unique()).await
    }
}

/// The filesystem running on the user space.
#[async_trait]
pub trait Filesystem<T, W: ?Sized>: Sync {
    /// Forget about inodes removed from the kernel's internal caches.
    #[allow(unused_variables)]
    async fn forget(&self, cx: &mut Context<'_>, forgets: &[Forget]) -> io::Result<()>
    where
        T: 'async_trait,
        W: 'async_trait,
    {
        Ok(())
    }

    /// Handle a FUSE request and reply its result to the kernel.
    #[allow(unused_variables)]
    async fn reply(
        &self,
        cx: &mut Context<'_>,
        writer: &mut ReplyWriter<'_, W>,
        op: Operation<'_, T>,
    ) -> io::Result<()>
    where
        T: Send + 'async_trait,
        W: Send + 'async_trait,
    {
        Ok(())
    }
}

impl<'a, F: ?Sized, T, W: ?Sized> Filesystem<T, W> for &'a F
where
    F: Filesystem<T, W>,
{
    #[inline]
    fn forget<'l1, 'l2, 'l3, 'l4, 'async_trait>(
        &'l1 self,
        cx: &'l2 mut Context<'l3>,
        forgets: &'l4 [Forget],
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'async_trait>>
    where
        'l1: 'async_trait,
        'l2: 'async_trait,
        'l3: 'async_trait,
        'l4: 'async_trait,
        T: 'async_trait,
        W: 'async_trait,
    {
        (**self).forget(cx, forgets)
    }

    #[inline]
    fn reply<'l1, 'l2, 'l3, 'l4, 'l5, 'l6, 'async_trait>(
        &'l1 self,
        cx: &'l2 mut Context<'l3>,
        writer: &'l4 mut ReplyWriter<'l5, W>,
        op: Operation<'l6, T>,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'async_trait>>
    where
        'l1: 'async_trait,
        'l2: 'async_trait,
        'l3: 'async_trait,
        'l4: 'async_trait,
        'l5: 'async_trait,
        'l6: 'async_trait,
        T: Send + 'async_trait,
        W: Send + 'async_trait,
    {
        (**self).reply(cx, writer, op)
    }
}

impl<F: ?Sized, T, W: ?Sized> Filesystem<T, W> for Box<F>
where
    F: Filesystem<T, W>,
{
    #[inline]
    fn forget<'l1, 'l2, 'l3, 'l4, 'async_trait>(
        &'l1 self,
        cx: &'l2 mut Context<'l3>,
        forgets: &'l4 [Forget],
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'async_trait>>
    where
        'l1: 'async_trait,
        'l2: 'async_trait,
        'l3: 'async_trait,
        'l4: 'async_trait,
        T: 'async_trait,
        W: 'async_trait,
    {
        (**self).forget(cx, forgets)
    }

    #[inline]
    fn reply<'l1, 'l2, 'l3, 'l4, 'l5, 'l6, 'async_trait>(
        &'l1 self,
        cx: &'l2 mut Context<'l3>,
        writer: &'l4 mut ReplyWriter<'l5, W>,
        op: Operation<'l6, T>,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'async_trait>>
    where
        'l1: 'async_trait,
        'l2: 'async_trait,
        'l3: 'async_trait,
        'l4: 'async_trait,
        'l5: 'async_trait,
        'l6: 'async_trait,
        T: Send + 'async_trait,
        W: Send + 'async_trait,
    {
        (**self).reply(cx, writer, op)
    }
}

impl<F: ?Sized, T, W: ?Sized> Filesystem<T, W> for std::sync::Arc<F>
where
    F: Filesystem<T, W> + Send,
{
    #[inline]
    fn forget<'l1, 'l2, 'l3, 'l4, 'async_trait>(
        &'l1 self,
        cx: &'l2 mut Context<'l3>,
        forgets: &'l4 [Forget],
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'async_trait>>
    where
        'l1: 'async_trait,
        'l2: 'async_trait,
        'l3: 'async_trait,
        'l4: 'async_trait,
        T: 'async_trait,
        W: 'async_trait,
    {
        (**self).forget(cx, forgets)
    }

    #[inline]
    fn reply<'l1, 'l2, 'l3, 'l4, 'l5, 'l6, 'async_trait>(
        &'l1 self,
        cx: &'l2 mut Context<'l3>,
        writer: &'l4 mut ReplyWriter<'l5, W>,
        op: Operation<'l6, T>,
    ) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'async_trait>>
    where
        'l1: 'async_trait,
        'l2: 'async_trait,
        'l3: 'async_trait,
        'l4: 'async_trait,
        'l5: 'async_trait,
        'l6: 'async_trait,
        T: Send + 'async_trait,
        W: Send + 'async_trait,
    {
        (**self).reply(cx, writer, op)
    }
}
