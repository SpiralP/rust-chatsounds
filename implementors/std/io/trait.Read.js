(function() {var implementors = {};
implementors["alsa"] = [{"text":"impl&lt;'a, S:&nbsp;Copy&gt; Read for IO&lt;'a, S&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for IO&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["base64"] = [{"text":"impl&lt;'a, R:&nbsp;Read&gt; Read for DecoderReader&lt;'a, R&gt;","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl&lt;B:&nbsp;Buf + Sized&gt; Read for Reader&lt;B&gt;","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl&lt;T&gt; Read for AllowStdIo&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Read,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["mio"] = [{"text":"impl Read for TcpStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a TcpStream","synthetic":false,"types":[]}];
implementors["mio_uds"] = [{"text":"impl Read for UnixStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a UnixStream","synthetic":false,"types":[]}];
implementors["native_tls"] = [{"text":"impl&lt;S:&nbsp;Read + Write&gt; Read for TlsStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl&lt;S:&nbsp;Read + Write&gt; Read for SslStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["rand_core"] = [{"text":"impl Read for dyn RngCore","synthetic":false,"types":[]}];
implementors["rmp_serde"] = [{"text":"impl&lt;R:&nbsp;Read&gt; Read for ReadReader&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;AsRef&lt;[u8]&gt; + ?Sized&gt; Read for ReadRefReader&lt;'a, T&gt;","synthetic":false,"types":[]}];
implementors["socket2"] = [{"text":"impl Read for Socket","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a Socket","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()