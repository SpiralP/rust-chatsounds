(function() {var implementors = {};
implementors["arrayvec"] = [{"text":"impl&lt;A&gt; AsRef&lt;str&gt; for ArrayString&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array&lt;Item = u8&gt; + Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; AsRef&lt;[&lt;A as Array&gt;::Item]&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl AsRef&lt;[u8]&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl AsRef&lt;[u8]&gt; for BytesMut","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T&gt; AsRef&lt;T&gt; for Owned&lt;T&gt;","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R, Target&gt; AsRef&lt;Target&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsRef&lt;Target&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsRef&lt;Target&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;L, R&gt; AsRef&lt;str&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsRef&lt;str&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsRef&lt;str&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;L, R, Target&gt; AsRef&lt;[Target]&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsRef&lt;[Target]&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsRef&lt;[Target]&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl&lt;T:&nbsp;AsRef&lt;[u8]&gt;&gt; AsRef&lt;[u8]&gt; for Window&lt;T&gt;","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; AsRef&lt;[T]&gt; for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 1]&gt; for GenericArray&lt;T, U1&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 2]&gt; for GenericArray&lt;T, U2&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 3]&gt; for GenericArray&lt;T, U3&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 4]&gt; for GenericArray&lt;T, U4&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 5]&gt; for GenericArray&lt;T, U5&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 6]&gt; for GenericArray&lt;T, U6&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 7]&gt; for GenericArray&lt;T, U7&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 8]&gt; for GenericArray&lt;T, U8&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 9]&gt; for GenericArray&lt;T, U9&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 10]&gt; for GenericArray&lt;T, U10&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 11]&gt; for GenericArray&lt;T, U11&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 12]&gt; for GenericArray&lt;T, U12&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 13]&gt; for GenericArray&lt;T, U13&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 14]&gt; for GenericArray&lt;T, U14&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 15]&gt; for GenericArray&lt;T, U15&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 16]&gt; for GenericArray&lt;T, U16&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 17]&gt; for GenericArray&lt;T, U17&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 18]&gt; for GenericArray&lt;T, U18&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 19]&gt; for GenericArray&lt;T, U19&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 20]&gt; for GenericArray&lt;T, U20&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 21]&gt; for GenericArray&lt;T, U21&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 22]&gt; for GenericArray&lt;T, U22&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 23]&gt; for GenericArray&lt;T, U23&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 24]&gt; for GenericArray&lt;T, U24&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 25]&gt; for GenericArray&lt;T, U25&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 26]&gt; for GenericArray&lt;T, U26&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 27]&gt; for GenericArray&lt;T, U27&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 28]&gt; for GenericArray&lt;T, U28&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 29]&gt; for GenericArray&lt;T, U29&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 30]&gt; for GenericArray&lt;T, U30&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 31]&gt; for GenericArray&lt;T, U31&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;[T; 32]&gt; for GenericArray&lt;T, U32&gt;","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl AsRef&lt;str&gt; for HeaderName","synthetic":false,"types":[]},{"text":"impl AsRef&lt;[u8]&gt; for HeaderName","synthetic":false,"types":[]},{"text":"impl AsRef&lt;[u8]&gt; for HeaderValue","synthetic":false,"types":[]},{"text":"impl AsRef&lt;str&gt; for Method","synthetic":false,"types":[]},{"text":"impl AsRef&lt;str&gt; for Authority","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;str&gt; for Port&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsRef&lt;str&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl AsRef&lt;str&gt; for Scheme","synthetic":false,"types":[]}];
implementors["mime"] = [{"text":"impl AsRef&lt;str&gt; for Mime","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; AsRef&lt;str&gt; for Name&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl AsRef&lt;str&gt; for Signal","synthetic":false,"types":[]},{"text":"impl AsRef&lt;sigset_t&gt; for SigSet","synthetic":false,"types":[]},{"text":"impl AsRef&lt;timespec&gt; for TimeSpec","synthetic":false,"types":[]},{"text":"impl AsRef&lt;timeval&gt; for TimeVal","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl AsRef&lt;Asn1GeneralizedTimeRef&gt; for Asn1GeneralizedTime","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Asn1TimeRef&gt; for Asn1Time","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Asn1StringRef&gt; for Asn1String","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Asn1IntegerRef&gt; for Asn1Integer","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Asn1BitStringRef&gt; for Asn1BitString","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Asn1ObjectRef&gt; for Asn1Object","synthetic":false,"types":[]},{"text":"impl AsRef&lt;BigNumContextRef&gt; for BigNumContext","synthetic":false,"types":[]},{"text":"impl AsRef&lt;BigNumRef&gt; for BigNum","synthetic":false,"types":[]},{"text":"impl AsRef&lt;CmsContentInfoRef&gt; for CmsContentInfo","synthetic":false,"types":[]},{"text":"impl AsRef&lt;ConfRef&gt; for Conf","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;DhRef&lt;T&gt;&gt; for Dh&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;DsaRef&lt;T&gt;&gt; for Dsa&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl AsRef&lt;EcGroupRef&gt; for EcGroup","synthetic":false,"types":[]},{"text":"impl AsRef&lt;EcPointRef&gt; for EcPoint","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;EcKeyRef&lt;T&gt;&gt; for EcKey&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl AsRef&lt;EcdsaSigRef&gt; for EcdsaSig","synthetic":false,"types":[]},{"text":"impl AsRef&lt;[u8]&gt; for DigestBytes","synthetic":false,"types":[]},{"text":"impl AsRef&lt;OcspBasicResponseRef&gt; for OcspBasicResponse","synthetic":false,"types":[]},{"text":"impl AsRef&lt;OcspCertIdRef&gt; for OcspCertId","synthetic":false,"types":[]},{"text":"impl AsRef&lt;OcspResponseRef&gt; for OcspResponse","synthetic":false,"types":[]},{"text":"impl AsRef&lt;OcspRequestRef&gt; for OcspRequest","synthetic":false,"types":[]},{"text":"impl AsRef&lt;OcspOneReqRef&gt; for OcspOneReq","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Pkcs12Ref&gt; for Pkcs12","synthetic":false,"types":[]},{"text":"impl AsRef&lt;Pkcs7Ref&gt; for Pkcs7","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;PKeyRef&lt;T&gt;&gt; for PKey&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; AsRef&lt;RsaRef&lt;T&gt;&gt; for Rsa&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl AsRef&lt;SrtpProtectionProfileRef&gt; for SrtpProtectionProfile","synthetic":false,"types":[]},{"text":"impl AsRef&lt;SslContextRef&gt; for SslContext","synthetic":false,"types":[]},{"text":"impl AsRef&lt;SslSessionRef&gt; for SslSession","synthetic":false,"types":[]},{"text":"impl AsRef&lt;SslRef&gt; for Ssl","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Stackable&gt; AsRef&lt;StackRef&lt;T&gt;&gt; for Stack&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl AsRef&lt;OpensslStringRef&gt; for OpensslString","synthetic":false,"types":[]},{"text":"impl AsRef&lt;str&gt; for OpensslString","synthetic":false,"types":[]},{"text":"impl AsRef&lt;[u8]&gt; for OpensslString","synthetic":false,"types":[]},{"text":"impl AsRef&lt;str&gt; for OpensslStringRef","synthetic":false,"types":[]},{"text":"impl AsRef&lt;[u8]&gt; for OpensslStringRef","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509VerifyParamRef&gt; for X509VerifyParam","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509StoreBuilderRef&gt; for X509StoreBuilder","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509StoreRef&gt; for X509Store","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509StoreContextRef&gt; for X509StoreContext","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509Ref&gt; for X509","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509Ref&gt; for X509Ref","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509ExtensionRef&gt; for X509Extension","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509NameRef&gt; for X509Name","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509NameEntryRef&gt; for X509NameEntry","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509ReqRef&gt; for X509Req","synthetic":false,"types":[]},{"text":"impl AsRef&lt;GeneralNameRef&gt; for GeneralName","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509AlgorithmRef&gt; for X509Algorithm","synthetic":false,"types":[]},{"text":"impl AsRef&lt;X509ObjectRef&gt; for X509Object","synthetic":false,"types":[]}];
implementors["slice_deque"] = [{"text":"impl&lt;T&gt; AsRef&lt;[T]&gt; for SliceDeque&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; AsRef&lt;[&lt;A as Array&gt;::Item]&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; AsRef&lt;[&lt;A as Array&gt;::Item]&gt; for TinyVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["tokio"] = [{"text":"impl&lt;'_&gt; AsRef&lt;TcpStream&gt; for ReadHalf&lt;'_&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; AsRef&lt;TcpStream&gt; for WriteHalf&lt;'_&gt;","synthetic":false,"types":[]},{"text":"impl AsRef&lt;TcpStream&gt; for OwnedReadHalf","synthetic":false,"types":[]},{"text":"impl AsRef&lt;TcpStream&gt; for OwnedWriteHalf","synthetic":false,"types":[]},{"text":"impl AsRef&lt;UdpSocket&gt; for SendHalf","synthetic":false,"types":[]},{"text":"impl AsRef&lt;UdpSocket&gt; for RecvHalf","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; AsRef&lt;UnixStream&gt; for ReadHalf&lt;'_&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; AsRef&lt;UnixStream&gt; for WriteHalf&lt;'_&gt;","synthetic":false,"types":[]}];
implementors["tracing_core"] = [{"text":"impl AsRef&lt;str&gt; for Field","synthetic":false,"types":[]}];
implementors["unicase"] = [{"text":"impl&lt;S:&nbsp;AsRef&lt;str&gt;&gt; AsRef&lt;str&gt; for Ascii&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;AsRef&lt;str&gt;&gt; AsRef&lt;str&gt; for UniCase&lt;S&gt;","synthetic":false,"types":[]}];
implementors["url"] = [{"text":"impl AsRef&lt;str&gt; for Url","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()