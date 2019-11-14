(function() {var implementors = {};
implementors["polyfuse"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://docs.rs/futures-core/0.3.0/futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"polyfuse/notify/struct.RetrieveHandle.html\" title=\"struct polyfuse::notify::RetrieveHandle\">RetrieveHandle</a>&lt;T&gt;",synthetic:false,types:["polyfuse::notify::RetrieveHandle"]},{text:"impl <a class=\"trait\" href=\"https://docs.rs/futures-core/0.3.0/futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"polyfuse/struct.Interrupt.html\" title=\"struct polyfuse::Interrupt\">Interrupt</a>",synthetic:false,types:["polyfuse::session::Interrupt"]},];
implementors["polyfuse_tokio"] = [{text:"impl <a class=\"trait\" href=\"https://docs.rs/futures-core/0.3.0/futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"polyfuse_tokio/struct.RetrieveHandle.html\" title=\"struct polyfuse_tokio::RetrieveHandle\">RetrieveHandle</a>",synthetic:false,types:["polyfuse_tokio::server::RetrieveHandle"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()