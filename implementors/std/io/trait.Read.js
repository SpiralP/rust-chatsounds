(function() {var implementors = {};
implementors["bytes"] = [{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"bytes/buf/trait.Buf.html\" title=\"trait bytes::buf::Buf\">Buf</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"bytes/buf/ext/struct.Reader.html\" title=\"struct bytes::buf::ext::Reader\">Reader</a>&lt;B&gt;","synthetic":false,"types":["bytes::buf::ext::reader::Reader"]}];
implementors["futures_util"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"futures_util/io/struct.AllowStdIo.html\" title=\"struct futures_util::io::AllowStdIo\">AllowStdIo</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,&nbsp;</span>","synthetic":false,"types":["futures_util::io::allow_std::AllowStdIo"]}];
implementors["mio"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"mio/net/struct.TcpStream.html\" title=\"struct mio::net::TcpStream\">TcpStream</a>","synthetic":false,"types":["mio::net::tcp::TcpStream"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for &amp;'a <a class=\"struct\" href=\"mio/net/struct.TcpStream.html\" title=\"struct mio::net::TcpStream\">TcpStream</a>","synthetic":false,"types":["mio::net::tcp::TcpStream"]}];
implementors["mio_uds"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"mio_uds/struct.UnixStream.html\" title=\"struct mio_uds::UnixStream\">UnixStream</a>","synthetic":false,"types":["mio_uds::stream::UnixStream"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for &amp;'a <a class=\"struct\" href=\"mio_uds/struct.UnixStream.html\" title=\"struct mio_uds::UnixStream\">UnixStream</a>","synthetic":false,"types":["mio_uds::stream::UnixStream"]}];
implementors["native_tls"] = [{"text":"impl&lt;S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"native_tls/struct.TlsStream.html\" title=\"struct native_tls::TlsStream\">TlsStream</a>&lt;S&gt;","synthetic":false,"types":["native_tls::TlsStream"]}];
implementors["openssl"] = [{"text":"impl&lt;S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"openssl/ssl/struct.SslStream.html\" title=\"struct openssl::ssl::SslStream\">SslStream</a>&lt;S&gt;","synthetic":false,"types":["openssl::ssl::SslStream"]}];
implementors["rand_core"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for dyn <a class=\"trait\" href=\"rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a>","synthetic":false,"types":["rand_core::RngCore"]}];
implementors["rmp_serde"] = [{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"rmp_serde/decode/struct.ReadReader.html\" title=\"struct rmp_serde::decode::ReadReader\">ReadReader</a>&lt;R&gt;","synthetic":false,"types":["rmp_serde::decode::ReadReader"]},{"text":"impl&lt;'a, T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"rmp_serde/decode/struct.ReadRefReader.html\" title=\"struct rmp_serde::decode::ReadRefReader\">ReadRefReader</a>&lt;'a, T&gt;","synthetic":false,"types":["rmp_serde::decode::ReadRefReader"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()