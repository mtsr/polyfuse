(function() {var implementors = {};
implementors["polyfuse"] = [{text:"impl <a class=\"trait\" href=\"https://docs.rs/futures-core/0.3.0/futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"polyfuse/struct.Interrupt.html\" title=\"struct polyfuse::Interrupt\">Interrupt</a>",synthetic:false,types:["polyfuse::session::Interrupt"]},{text:"impl&lt;T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://docs.rs/futures-core/0.3.0/futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"polyfuse/struct.NotifyRetrieve.html\" title=\"struct polyfuse::NotifyRetrieve\">NotifyRetrieve</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"polyfuse/request/trait.Buffer.html\" title=\"trait polyfuse::request::Buffer\">Buffer</a>,&nbsp;</span>",synthetic:false,types:["polyfuse::session::NotifyRetrieve"]},];
implementors["polyfuse_tokio"] = [{text:"impl <a class=\"trait\" href=\"https://docs.rs/futures-core/0.3.0/futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"polyfuse_tokio/struct.NotifyHandle.html\" title=\"struct polyfuse_tokio::NotifyHandle\">NotifyHandle</a>",synthetic:false,types:["polyfuse_tokio::server::NotifyHandle"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()