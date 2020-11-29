(function() {var implementors = {};
implementors["alsa"] = [{"text":"impl Extend&lt;Flags&gt; for Flags","synthetic":false,"types":[]},{"text":"impl Extend&lt;PortCap&gt; for PortCap","synthetic":false,"types":[]},{"text":"impl Extend&lt;PortType&gt; for PortType","synthetic":false,"types":[]},{"text":"impl Extend&lt;Remove&gt; for Remove","synthetic":false,"types":[]}];
implementors["arrayvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Extend&lt;&lt;A as Array&gt;::Item&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["bitvec"] = [{"text":"impl&lt;O, T&gt; Extend&lt;bool&gt; for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Extend&lt;&amp;'a bool&gt; for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl Extend&lt;u8&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Extend&lt;&amp;'a u8&gt; for BytesMut","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R, A&gt; Extend&lt;A&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Extend&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Extend&lt;A&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl&lt;Fut:&nbsp;Future&gt; Extend&lt;Fut&gt; for FuturesOrdered&lt;Fut&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Fut&gt; Extend&lt;Fut&gt; for FuturesUnordered&lt;Fut&gt;","synthetic":false,"types":[]},{"text":"impl&lt;St:&nbsp;Stream + Unpin&gt; Extend&lt;St&gt; for SelectAll&lt;St&gt;","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;K, V, S&gt; Extend&lt;(K, V)&gt; for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Extend&lt;(&amp;'a K, &amp;'a V)&gt; for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; Extend&lt;T&gt; for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Extend&lt;&amp;'a T&gt; for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + Eq + Hash + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl&lt;T&gt; Extend&lt;(Option&lt;HeaderName&gt;, T)&gt; for HeaderMap&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Extend&lt;(HeaderName, T)&gt; for HeaderMap&lt;T&gt;","synthetic":false,"types":[]}];
implementors["indexmap"] = [{"text":"impl&lt;K, V, S&gt; Extend&lt;(K, V)&gt; for IndexMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Hash + Eq,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Extend&lt;(&amp;'a K, &amp;'a V)&gt; for IndexMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Hash + Eq + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; Extend&lt;T&gt; for IndexSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Hash + Eq,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Extend&lt;&amp;'a T&gt; for IndexSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Hash + Eq + Copy + 'a,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl Extend&lt;AtFlags&gt; for AtFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;OFlag&gt; for OFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;SealFlag&gt; for SealFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;SpliceFFlags&gt; for SpliceFFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;FallocateFlags&gt; for FallocateFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;ModuleInitFlags&gt; for ModuleInitFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;DeleteModuleFlags&gt; for DeleteModuleFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MntFlags&gt; for MntFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MQ_OFlag&gt; for MQ_OFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;InterfaceFlags&gt; for InterfaceFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;PollFlags&gt; for PollFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;CloneFlags&gt; for CloneFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;EpollFlags&gt; for EpollFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;EpollCreateFlags&gt; for EpollCreateFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;EfdFlags&gt; for EfdFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MemFdCreateFlag&gt; for MemFdCreateFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;ProtFlags&gt; for ProtFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MapFlags&gt; for MapFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MlockAllFlags&gt; for MlockAllFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;Options&gt; for Options","synthetic":false,"types":[]},{"text":"impl Extend&lt;QuotaValidFlags&gt; for QuotaValidFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;SaFlags&gt; for SaFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;SfdFlags&gt; for SfdFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;SockFlag&gt; for SockFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;MsgFlags&gt; for MsgFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;SFlag&gt; for SFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;Mode&gt; for Mode","synthetic":false,"types":[]},{"text":"impl Extend&lt;FsFlags&gt; for FsFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;InputFlags&gt; for InputFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;OutputFlags&gt; for OutputFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;ControlFlags&gt; for ControlFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;LocalFlags&gt; for LocalFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;WaitPidFlag&gt; for WaitPidFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;AddWatchFlags&gt; for AddWatchFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;InitFlags&gt; for InitFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;AccessFlags&gt; for AccessFlags","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl Extend&lt;CMSOptions&gt; for CMSOptions","synthetic":false,"types":[]},{"text":"impl Extend&lt;OcspFlag&gt; for OcspFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;Pkcs7Flags&gt; for Pkcs7Flags","synthetic":false,"types":[]},{"text":"impl Extend&lt;SslOptions&gt; for SslOptions","synthetic":false,"types":[]},{"text":"impl Extend&lt;SslMode&gt; for SslMode","synthetic":false,"types":[]},{"text":"impl Extend&lt;SslVerifyMode&gt; for SslVerifyMode","synthetic":false,"types":[]},{"text":"impl Extend&lt;SslSessionCacheMode&gt; for SslSessionCacheMode","synthetic":false,"types":[]},{"text":"impl Extend&lt;ExtensionContext&gt; for ExtensionContext","synthetic":false,"types":[]},{"text":"impl Extend&lt;ShutdownState&gt; for ShutdownState","synthetic":false,"types":[]},{"text":"impl Extend&lt;X509CheckFlags&gt; for X509CheckFlags","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Extend&lt;TokenTree&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl Extend&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl Extend&lt;(String, Value)&gt; for Map&lt;String, Value&gt;","synthetic":false,"types":[]}];
implementors["slice_deque"] = [{"text":"impl&lt;T&gt; Extend&lt;T&gt; for SliceDeque&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;'a + Copy&gt; Extend&lt;&amp;'a T&gt; for SliceDeque&lt;T&gt;","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Extend&lt;&lt;A as Array&gt;::Item&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;T, P&gt; Extend&lt;T&gt; for Punctuated&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, P&gt; Extend&lt;Pair&lt;T, P&gt;&gt; for Punctuated&lt;T, P&gt;","synthetic":false,"types":[]},{"text":"impl Extend&lt;Error&gt; for Error","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Extend&lt;&lt;A as Array&gt;::Item&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'s, T&gt; Extend&lt;T&gt; for SliceVec&lt;'s, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Extend&lt;&lt;A as Array&gt;::Item&gt; for TinyVec&lt;A&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()