(function() {var implementors = {};
implementors["alsa"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["arrayvec"] = [{"text":"impl&lt;T:&nbsp;Any&gt; Error for CapacityError&lt;T&gt;","synthetic":false,"types":[]}];
implementors["base64"] = [{"text":"impl Error for DecodeError","synthetic":false,"types":[]}];
implementors["chatsounds"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["claxon"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["cpal"] = [{"text":"impl Error for HostUnavailable","synthetic":false,"types":[]},{"text":"impl Error for BackendSpecificError","synthetic":false,"types":[]},{"text":"impl Error for DevicesError","synthetic":false,"types":[]},{"text":"impl Error for DeviceNameError","synthetic":false,"types":[]},{"text":"impl Error for SupportedStreamConfigsError","synthetic":false,"types":[]},{"text":"impl Error for DefaultStreamConfigError","synthetic":false,"types":[]},{"text":"impl Error for BuildStreamError","synthetic":false,"types":[]},{"text":"impl Error for PlayStreamError","synthetic":false,"types":[]},{"text":"impl Error for PauseStreamError","synthetic":false,"types":[]},{"text":"impl Error for StreamError","synthetic":false,"types":[]}];
implementors["crossbeam_channel"] = [{"text":"impl&lt;T:&nbsp;Send&gt; Error for SendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Send&gt; Error for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Send&gt; Error for SendTimeoutError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for RecvError","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl Error for RecvTimeoutError","synthetic":false,"types":[]},{"text":"impl Error for TrySelectError","synthetic":false,"types":[]},{"text":"impl Error for SelectTimeoutError","synthetic":false,"types":[]}];
implementors["digest"] = [{"text":"impl Error for InvalidOutputSize","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; Error for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Error,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Error,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["error_chain"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["futures_channel"] = [{"text":"impl Error for SendError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Any&gt; Error for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl Error for Canceled","synthetic":false,"types":[]}];
implementors["futures_executor"] = [{"text":"impl Error for EnterError","synthetic":false,"types":[]}];
implementors["futures_task"] = [{"text":"impl Error for SpawnError","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl Error for Aborted","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Any, Item&gt; Error for ReuniteError&lt;T, Item&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Any&gt; Error for ReuniteError&lt;T&gt;","synthetic":false,"types":[]}];
implementors["getrandom"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["h2"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["hound"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl Error for InvalidHeaderName","synthetic":false,"types":[]},{"text":"impl Error for InvalidHeaderValue","synthetic":false,"types":[]},{"text":"impl Error for ToStrError","synthetic":false,"types":[]},{"text":"impl Error for InvalidMethod","synthetic":false,"types":[]},{"text":"impl Error for InvalidStatusCode","synthetic":false,"types":[]},{"text":"impl Error for InvalidUri","synthetic":false,"types":[]},{"text":"impl Error for InvalidUriParts","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["httparse"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["httpdate"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["hyper"] = [{"text":"impl Error for InvalidNameError","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["ipnet"] = [{"text":"impl Error for PrefixLenError","synthetic":false,"types":[]},{"text":"impl Error for AddrParseError","synthetic":false,"types":[]}];
implementors["lewton"] = [{"text":"impl Error for HeaderReadError","synthetic":false,"types":[]},{"text":"impl Error for AudioReadError","synthetic":false,"types":[]},{"text":"impl Error for VorbisError","synthetic":false,"types":[]}];
implementors["mime"] = [{"text":"impl Error for FromStrError","synthetic":false,"types":[]}];
implementors["minimp3"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["native_tls"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl&lt;S&gt; Error for HandshakeError&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Any + Debug,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl Error for Errno","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["nom"] = [{"text":"impl&lt;I:&nbsp;Debug + Display&gt; Error for Error&lt;I&gt;","synthetic":false,"types":[]},{"text":"impl&lt;E&gt; Error for Err&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Debug,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["ogg"] = [{"text":"impl Error for OggReadError","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl Error for ErrorStack","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;Debug&gt; Error for HandshakeError&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl Error for X509VerifyResult","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Error for LexError","synthetic":false,"types":[]}];
implementors["rand"] = [{"text":"impl Error for BernoulliError","synthetic":false,"types":[]},{"text":"impl Error for WeightedError","synthetic":false,"types":[]},{"text":"impl Error for ReadError","synthetic":false,"types":[]}];
implementors["rand_core"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["rayon_core"] = [{"text":"impl Error for ThreadPoolBuildError","synthetic":false,"types":[]}];
implementors["reqwest"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["rmp"] = [{"text":"impl&lt;'a&gt; Error for DecodeStringError&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Error for ValueReadError","synthetic":false,"types":[]},{"text":"impl Error for NumValueReadError","synthetic":false,"types":[]},{"text":"impl Error for ValueWriteError","synthetic":false,"types":[]}];
implementors["rmp_serde"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["rodio"] = [{"text":"impl Error for PlayError","synthetic":false,"types":[]},{"text":"impl Error for StreamError","synthetic":false,"types":[]},{"text":"impl Error for DecoderError","synthetic":false,"types":[]}];
implementors["serde"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["serde_urlencoded"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["tokio"] = [{"text":"impl Error for ReuniteError","synthetic":false,"types":[]},{"text":"impl Error for ReuniteError","synthetic":false,"types":[]},{"text":"impl Error for JoinError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Error for SendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for RecvError","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Error for SendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Error for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for RecvError","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Error for SendTimeoutError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for TryLockError","synthetic":false,"types":[]},{"text":"impl Error for RecvError","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Error for SendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for RecvError","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Elapsed","synthetic":false,"types":[]}];
implementors["tokio_util"] = [{"text":"impl Error for LengthDelimitedCodecError","synthetic":false,"types":[]},{"text":"impl Error for LinesCodecError","synthetic":false,"types":[]}];
implementors["tracing_core"] = [{"text":"impl Error for SetGlobalDefaultError","synthetic":false,"types":[]},{"text":"impl Error for ParseLevelError","synthetic":false,"types":[]},{"text":"impl Error for ParseLevelFilterError","synthetic":false,"types":[]}];
implementors["url"] = [{"text":"impl Error for ParseError","synthetic":false,"types":[]}];
implementors["void"] = [{"text":"impl Error for Void","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()