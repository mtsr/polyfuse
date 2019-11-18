(function() {var implementors = {};
implementors["polyfuse"] = [{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/struct.FileAttr.html\" title=\"struct polyfuse::FileAttr\">FileAttr</a>",synthetic:true,types:["polyfuse::common::FileAttr"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/struct.FileLock.html\" title=\"struct polyfuse::FileLock\">FileLock</a>",synthetic:true,types:["polyfuse::common::FileLock"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/struct.Forget.html\" title=\"struct polyfuse::Forget\">Forget</a>",synthetic:true,types:["polyfuse::common::Forget"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/struct.StatFs.html\" title=\"struct polyfuse::StatFs\">StatFs</a>",synthetic:true,types:["polyfuse::common::StatFs"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/struct.DirEntry.html\" title=\"struct polyfuse::DirEntry\">DirEntry</a>",synthetic:true,types:["polyfuse::dirent::DirEntry"]},{text:"impl&lt;'a, W:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; Freeze for <a class=\"struct\" href=\"polyfuse/struct.Context.html\" title=\"struct polyfuse::Context\">Context</a>&lt;'a, W&gt;",synthetic:true,types:["polyfuse::fs::Context"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/struct.CapabilityFlags.html\" title=\"struct polyfuse::CapabilityFlags\">CapabilityFlags</a>",synthetic:true,types:["polyfuse::init::CapabilityFlags"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/struct.ConnectionInfo.html\" title=\"struct polyfuse::ConnectionInfo\">ConnectionInfo</a>",synthetic:true,types:["polyfuse::init::ConnectionInfo"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/struct.SessionInitializer.html\" title=\"struct polyfuse::SessionInitializer\">SessionInitializer</a>",synthetic:true,types:["polyfuse::init::SessionInitializer"]},{text:"impl&lt;T&gt; !Freeze for <a class=\"struct\" href=\"polyfuse/notify/struct.Notifier.html\" title=\"struct polyfuse::notify::Notifier\">Notifier</a>&lt;T&gt;",synthetic:true,types:["polyfuse::notify::Notifier"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/struct.Interrupt.html\" title=\"struct polyfuse::Interrupt\">Interrupt</a>",synthetic:true,types:["polyfuse::session::Interrupt"]},{text:"impl !Freeze for <a class=\"struct\" href=\"polyfuse/struct.Session.html\" title=\"struct polyfuse::Session\">Session</a>",synthetic:true,types:["polyfuse::session::Session"]},{text:"impl&lt;'a, T&gt; Freeze for <a class=\"enum\" href=\"polyfuse/enum.Operation.html\" title=\"enum polyfuse::Operation\">Operation</a>&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Freeze,&nbsp;</span>",synthetic:true,types:["polyfuse::fs::Operation"]},{text:"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"polyfuse/notify/struct.RetrieveHandle.html\" title=\"struct polyfuse::notify::RetrieveHandle\">RetrieveHandle</a>&lt;T&gt;",synthetic:true,types:["polyfuse::notify::RetrieveHandle"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyEmpty.html\" title=\"struct polyfuse::reply::ReplyEmpty\">ReplyEmpty</a>",synthetic:true,types:["polyfuse::reply::ReplyEmpty"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyData.html\" title=\"struct polyfuse::reply::ReplyData\">ReplyData</a>",synthetic:true,types:["polyfuse::reply::ReplyData"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyAttr.html\" title=\"struct polyfuse::reply::ReplyAttr\">ReplyAttr</a>",synthetic:true,types:["polyfuse::reply::ReplyAttr"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyEntry.html\" title=\"struct polyfuse::reply::ReplyEntry\">ReplyEntry</a>",synthetic:true,types:["polyfuse::reply::ReplyEntry"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyReadlink.html\" title=\"struct polyfuse::reply::ReplyReadlink\">ReplyReadlink</a>",synthetic:true,types:["polyfuse::reply::ReplyReadlink"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyOpen.html\" title=\"struct polyfuse::reply::ReplyOpen\">ReplyOpen</a>",synthetic:true,types:["polyfuse::reply::ReplyOpen"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyWrite.html\" title=\"struct polyfuse::reply::ReplyWrite\">ReplyWrite</a>",synthetic:true,types:["polyfuse::reply::ReplyWrite"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyOpendir.html\" title=\"struct polyfuse::reply::ReplyOpendir\">ReplyOpendir</a>",synthetic:true,types:["polyfuse::reply::ReplyOpendir"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyXattr.html\" title=\"struct polyfuse::reply::ReplyXattr\">ReplyXattr</a>",synthetic:true,types:["polyfuse::reply::ReplyXattr"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyStatfs.html\" title=\"struct polyfuse::reply::ReplyStatfs\">ReplyStatfs</a>",synthetic:true,types:["polyfuse::reply::ReplyStatfs"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyLk.html\" title=\"struct polyfuse::reply::ReplyLk\">ReplyLk</a>",synthetic:true,types:["polyfuse::reply::ReplyLk"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyCreate.html\" title=\"struct polyfuse::reply::ReplyCreate\">ReplyCreate</a>",synthetic:true,types:["polyfuse::reply::ReplyCreate"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyBmap.html\" title=\"struct polyfuse::reply::ReplyBmap\">ReplyBmap</a>",synthetic:true,types:["polyfuse::reply::ReplyBmap"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/reply/struct.ReplyPoll.html\" title=\"struct polyfuse::reply::ReplyPoll\">ReplyPoll</a>",synthetic:true,types:["polyfuse::reply::ReplyPoll"]},{text:"impl !Freeze for <a class=\"struct\" href=\"polyfuse/request/struct.BytesBuffer.html\" title=\"struct polyfuse::request::BytesBuffer\">BytesBuffer</a>",synthetic:true,types:["polyfuse::request::BytesBuffer"]},{text:"impl&lt;'a, T&gt; Freeze for <a class=\"struct\" href=\"polyfuse/request/struct.Request.html\" title=\"struct polyfuse::request::Request\">Request</a>&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Freeze,&nbsp;</span>",synthetic:true,types:["polyfuse::request::Request"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse/request/struct.RequestHeader.html\" title=\"struct polyfuse::request::RequestHeader\">RequestHeader</a>",synthetic:true,types:["polyfuse::request::RequestHeader"]},];
implementors["polyfuse_sys"] = [{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_attr.html\" title=\"struct polyfuse_sys::kernel::fuse_attr\">fuse_attr</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_attr"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_dirent.html\" title=\"struct polyfuse_sys::kernel::fuse_dirent\">fuse_dirent</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_dirent"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_direntplus.html\" title=\"struct polyfuse_sys::kernel::fuse_direntplus\">fuse_direntplus</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_direntplus"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_kstatfs.html\" title=\"struct polyfuse_sys::kernel::fuse_kstatfs\">fuse_kstatfs</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_kstatfs"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_file_lock.html\" title=\"struct polyfuse_sys::kernel::fuse_file_lock\">fuse_file_lock</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_file_lock"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_in_header.html\" title=\"struct polyfuse_sys::kernel::fuse_in_header\">fuse_in_header</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_in_header"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_init_in.html\" title=\"struct polyfuse_sys::kernel::fuse_init_in\">fuse_init_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_init_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_forget_in.html\" title=\"struct polyfuse_sys::kernel::fuse_forget_in\">fuse_forget_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_forget_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_getattr_in.html\" title=\"struct polyfuse_sys::kernel::fuse_getattr_in\">fuse_getattr_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_getattr_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_setattr_in.html\" title=\"struct polyfuse_sys::kernel::fuse_setattr_in\">fuse_setattr_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_setattr_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_mknod_in.html\" title=\"struct polyfuse_sys::kernel::fuse_mknod_in\">fuse_mknod_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_mknod_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_mkdir_in.html\" title=\"struct polyfuse_sys::kernel::fuse_mkdir_in\">fuse_mkdir_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_mkdir_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_rename_in.html\" title=\"struct polyfuse_sys::kernel::fuse_rename_in\">fuse_rename_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_rename_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_link_in.html\" title=\"struct polyfuse_sys::kernel::fuse_link_in\">fuse_link_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_link_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_open_in.html\" title=\"struct polyfuse_sys::kernel::fuse_open_in\">fuse_open_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_open_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_read_in.html\" title=\"struct polyfuse_sys::kernel::fuse_read_in\">fuse_read_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_read_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_write_in.html\" title=\"struct polyfuse_sys::kernel::fuse_write_in\">fuse_write_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_write_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_flush_in.html\" title=\"struct polyfuse_sys::kernel::fuse_flush_in\">fuse_flush_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_flush_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_release_in.html\" title=\"struct polyfuse_sys::kernel::fuse_release_in\">fuse_release_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_release_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_fsync_in.html\" title=\"struct polyfuse_sys::kernel::fuse_fsync_in\">fuse_fsync_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_fsync_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_getxattr_in.html\" title=\"struct polyfuse_sys::kernel::fuse_getxattr_in\">fuse_getxattr_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_getxattr_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_setxattr_in.html\" title=\"struct polyfuse_sys::kernel::fuse_setxattr_in\">fuse_setxattr_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_setxattr_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_lk_in.html\" title=\"struct polyfuse_sys::kernel::fuse_lk_in\">fuse_lk_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_lk_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_access_in.html\" title=\"struct polyfuse_sys::kernel::fuse_access_in\">fuse_access_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_access_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_create_in.html\" title=\"struct polyfuse_sys::kernel::fuse_create_in\">fuse_create_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_create_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_bmap_in.html\" title=\"struct polyfuse_sys::kernel::fuse_bmap_in\">fuse_bmap_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_bmap_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_out_header.html\" title=\"struct polyfuse_sys::kernel::fuse_out_header\">fuse_out_header</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_out_header"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_attr_out.html\" title=\"struct polyfuse_sys::kernel::fuse_attr_out\">fuse_attr_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_attr_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_entry_out.html\" title=\"struct polyfuse_sys::kernel::fuse_entry_out\">fuse_entry_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_entry_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_init_out.html\" title=\"struct polyfuse_sys::kernel::fuse_init_out\">fuse_init_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_init_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_getxattr_out.html\" title=\"struct polyfuse_sys::kernel::fuse_getxattr_out\">fuse_getxattr_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_getxattr_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_open_out.html\" title=\"struct polyfuse_sys::kernel::fuse_open_out\">fuse_open_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_open_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_write_out.html\" title=\"struct polyfuse_sys::kernel::fuse_write_out\">fuse_write_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_write_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_statfs_out.html\" title=\"struct polyfuse_sys::kernel::fuse_statfs_out\">fuse_statfs_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_statfs_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_lk_out.html\" title=\"struct polyfuse_sys::kernel::fuse_lk_out\">fuse_lk_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_lk_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_bmap_out.html\" title=\"struct polyfuse_sys::kernel::fuse_bmap_out\">fuse_bmap_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_bmap_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_ioctl_in.html\" title=\"struct polyfuse_sys::kernel::fuse_ioctl_in\">fuse_ioctl_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_ioctl_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_ioctl_out.html\" title=\"struct polyfuse_sys::kernel::fuse_ioctl_out\">fuse_ioctl_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_ioctl_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_ioctl_iovec.html\" title=\"struct polyfuse_sys::kernel::fuse_ioctl_iovec\">fuse_ioctl_iovec</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_ioctl_iovec"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_poll_in.html\" title=\"struct polyfuse_sys::kernel::fuse_poll_in\">fuse_poll_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_poll_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_poll_out.html\" title=\"struct polyfuse_sys::kernel::fuse_poll_out\">fuse_poll_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_poll_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_interrupt_in.html\" title=\"struct polyfuse_sys::kernel::fuse_interrupt_in\">fuse_interrupt_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_interrupt_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_fallocate_in.html\" title=\"struct polyfuse_sys::kernel::fuse_fallocate_in\">fuse_fallocate_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_fallocate_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_batch_forget_in.html\" title=\"struct polyfuse_sys::kernel::fuse_batch_forget_in\">fuse_batch_forget_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_batch_forget_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_forget_one.html\" title=\"struct polyfuse_sys::kernel::fuse_forget_one\">fuse_forget_one</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_forget_one"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_rename2_in.html\" title=\"struct polyfuse_sys::kernel::fuse_rename2_in\">fuse_rename2_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_rename2_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_lseek_in.html\" title=\"struct polyfuse_sys::kernel::fuse_lseek_in\">fuse_lseek_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_lseek_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_lseek_out.html\" title=\"struct polyfuse_sys::kernel::fuse_lseek_out\">fuse_lseek_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_lseek_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_copy_file_range_in.html\" title=\"struct polyfuse_sys::kernel::fuse_copy_file_range_in\">fuse_copy_file_range_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_copy_file_range_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_notify_poll_wakeup_out.html\" title=\"struct polyfuse_sys::kernel::fuse_notify_poll_wakeup_out\">fuse_notify_poll_wakeup_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_notify_poll_wakeup_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_notify_inval_inode_out.html\" title=\"struct polyfuse_sys::kernel::fuse_notify_inval_inode_out\">fuse_notify_inval_inode_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_notify_inval_inode_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_notify_inval_entry_out.html\" title=\"struct polyfuse_sys::kernel::fuse_notify_inval_entry_out\">fuse_notify_inval_entry_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_notify_inval_entry_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_notify_delete_out.html\" title=\"struct polyfuse_sys::kernel::fuse_notify_delete_out\">fuse_notify_delete_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_notify_delete_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_notify_store_out.html\" title=\"struct polyfuse_sys::kernel::fuse_notify_store_out\">fuse_notify_store_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_notify_store_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_notify_retrieve_out.html\" title=\"struct polyfuse_sys::kernel::fuse_notify_retrieve_out\">fuse_notify_retrieve_out</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_notify_retrieve_out"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.fuse_notify_retrieve_in.html\" title=\"struct polyfuse_sys::kernel::fuse_notify_retrieve_in\">fuse_notify_retrieve_in</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_notify_retrieve_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.cuse_init_in.html\" title=\"struct polyfuse_sys::kernel::cuse_init_in\">cuse_init_in</a>",synthetic:true,types:["polyfuse_sys::kernel::cuse_init_in"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_sys/kernel/struct.cuse_init_out.html\" title=\"struct polyfuse_sys::kernel::cuse_init_out\">cuse_init_out</a>",synthetic:true,types:["polyfuse_sys::kernel::cuse_init_out"]},{text:"impl Freeze for <a class=\"enum\" href=\"polyfuse_sys/kernel/enum.fuse_opcode.html\" title=\"enum polyfuse_sys::kernel::fuse_opcode\">fuse_opcode</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_opcode"]},{text:"impl Freeze for <a class=\"enum\" href=\"polyfuse_sys/kernel/enum.fuse_notify_code.html\" title=\"enum polyfuse_sys::kernel::fuse_notify_code\">fuse_notify_code</a>",synthetic:true,types:["polyfuse_sys::kernel::fuse_notify_code"]},];
implementors["polyfuse_tokio"] = [{text:"impl !Freeze for <a class=\"struct\" href=\"polyfuse_tokio/struct.Channel.html\" title=\"struct polyfuse_tokio::Channel\">Channel</a>",synthetic:true,types:["polyfuse_tokio::channel::Channel"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_tokio/struct.MountOptions.html\" title=\"struct polyfuse_tokio::MountOptions\">MountOptions</a>",synthetic:true,types:["polyfuse_tokio::mount::MountOptions"]},{text:"impl !Freeze for <a class=\"struct\" href=\"polyfuse_tokio/struct.Notifier.html\" title=\"struct polyfuse_tokio::Notifier\">Notifier</a>",synthetic:true,types:["polyfuse_tokio::server::Notifier"]},{text:"impl Freeze for <a class=\"struct\" href=\"polyfuse_tokio/struct.RetrieveHandle.html\" title=\"struct polyfuse_tokio::RetrieveHandle\">RetrieveHandle</a>",synthetic:true,types:["polyfuse_tokio::server::RetrieveHandle"]},{text:"impl !Freeze for <a class=\"struct\" href=\"polyfuse_tokio/struct.Server.html\" title=\"struct polyfuse_tokio::Server\">Server</a>",synthetic:true,types:["polyfuse_tokio::server::Server"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()