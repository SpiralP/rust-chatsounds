(function() {var implementors = {};
implementors["futures_channel"] = [{"text":"impl&lt;T, '_&gt; Future for Cancellation&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Future for Receiver&lt;T&gt;","synthetic":false,"types":[]}];
implementors["futures_task"] = [{"text":"impl&lt;T, '_&gt; Future for LocalFutureObj&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, '_&gt; Future for FutureObj&lt;'_, T&gt;","synthetic":false,"types":[]}];
implementors["h2"] = [{"text":"impl&lt;B&gt; Future for ReadySendRequest&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Buf + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, B&gt; Future for Connection&lt;T, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncRead + AsyncWrite + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Buf + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Future for ResponseFuture","synthetic":false,"types":[]},{"text":"impl Future for PushedResponseFuture","synthetic":false,"types":[]},{"text":"impl&lt;T, B:&nbsp;Buf&gt; Future for Handshake&lt;T, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncRead + AsyncWrite + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Buf + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["http_body"] = [{"text":"impl&lt;'a, T:&nbsp;Body + Unpin + ?Sized&gt; Future for Data&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Body + Unpin + ?Sized&gt; Future for Trailers&lt;'a, T&gt;","synthetic":false,"types":[]}];
implementors["hyper"] = [{"text":"impl&lt;T, B&gt; Future for Connection&lt;T, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncRead + AsyncWrite + Unpin + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: HttpBody + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Data: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Error: Into&lt;Box&lt;dyn StdError + Send + Sync&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Future for ResponseFuture","synthetic":false,"types":[]},{"text":"impl Future for GaiFuture","synthetic":false,"types":[]},{"text":"impl Future for ResponseFuture","synthetic":false,"types":[]},{"text":"impl&lt;I, B, S, E&gt; Future for Connection&lt;I, S, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: HttpService&lt;Body, ResBody = B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::Error: Into&lt;Box&lt;dyn StdError + Send + Sync&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: AsyncRead + AsyncWrite + Unpin + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: HttpBody + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Error: Into&lt;Box&lt;dyn StdError + Send + Sync&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: H2Exec&lt;S::Future, B&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I, F, S, FE, E, B&gt; Future for Connecting&lt;I, F, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: AsyncRead + AsyncWrite + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Future&lt;Output = Result&lt;S, FE&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: HttpService&lt;Body, ResBody = B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: HttpBody + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Error: Into&lt;Box&lt;dyn StdError + Send + Sync&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: H2Exec&lt;S::Future, B&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I, IO, IE, S, B, E&gt; Future for Server&lt;I, S, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Accept&lt;Conn = IO, Error = IE&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;IE: Into&lt;Box&lt;dyn StdError + Send + Sync&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: MakeServiceRef&lt;IO, Body, ResBody = B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::Error: Into&lt;Box&lt;dyn StdError + Send + Sync&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: HttpBody + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Error: Into&lt;Box&lt;dyn StdError + Send + Sync&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: H2Exec&lt;&lt;S::Service as HttpService&lt;Body&gt;&gt;::Future, B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: NewSvcExec&lt;IO, S::Future, S::Service, E, NoopWatcher&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Future for OnUpgrade","synthetic":false,"types":[]}];
implementors["hyper_tls"] = [{"text":"impl&lt;T:&nbsp;AsyncRead + AsyncWrite + Unpin&gt; Future for HttpsConnecting&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tokio"] = [{"text":"impl&lt;S:&nbsp;?Sized, '_&gt; Future for Seek&lt;'_, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: AsyncSeek + Unpin,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;?Sized, W:&nbsp;?Sized, '_&gt; Future for Copy&lt;'_, R, W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsyncRead + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: AsyncWrite + Unpin,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Future for Child","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Future for JoinHandle&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Future for Receiver&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Future for LocalSet","synthetic":false,"types":[]},{"text":"impl Future for Delay","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Future for Timeout&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Future,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()