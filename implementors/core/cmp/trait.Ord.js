(function() {var implementors = {};
implementors["alsa"] = [{"text":"impl Ord for Direction","synthetic":false,"types":[]},{"text":"impl Ord for ValueOr","synthetic":false,"types":[]},{"text":"impl Ord for Round","synthetic":false,"types":[]},{"text":"impl Ord for Card","synthetic":false,"types":[]},{"text":"impl Ord for ElemIface","synthetic":false,"types":[]},{"text":"impl Ord for ElemType","synthetic":false,"types":[]},{"text":"impl Ord for State","synthetic":false,"types":[]},{"text":"impl Ord for Format","synthetic":false,"types":[]},{"text":"impl Ord for Access","synthetic":false,"types":[]},{"text":"impl Ord for TstampType","synthetic":false,"types":[]},{"text":"impl Ord for Flags","synthetic":false,"types":[]},{"text":"impl Ord for MilliBel","synthetic":false,"types":[]},{"text":"impl Ord for SelemChannelId","synthetic":false,"types":[]},{"text":"impl Ord for PortCap","synthetic":false,"types":[]},{"text":"impl Ord for PortType","synthetic":false,"types":[]},{"text":"impl Ord for Remove","synthetic":false,"types":[]},{"text":"impl Ord for Addr","synthetic":false,"types":[]},{"text":"impl Ord for EvNote","synthetic":false,"types":[]},{"text":"impl Ord for EvCtrl","synthetic":false,"types":[]},{"text":"impl Ord for Connect","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord&gt; Ord for EvQueueControl&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Ord for EvResult","synthetic":false,"types":[]},{"text":"impl Ord for EventType","synthetic":false,"types":[]},{"text":"impl Ord for ChmapType","synthetic":false,"types":[]},{"text":"impl Ord for ChmapPosition","synthetic":false,"types":[]}];
implementors["alsa_sys"] = [{"text":"impl&lt;Storage:&nbsp;Ord, Align:&nbsp;Ord&gt; Ord for __BindgenBitfieldUnit&lt;Storage, Align&gt;","synthetic":false,"types":[]}];
implementors["arc_swap"] = [{"text":"impl&lt;T:&nbsp;Ord&gt; Ord for ConstantDeref&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord&gt; Ord for Constant&lt;T&gt;","synthetic":false,"types":[]}];
implementors["arrayvec"] = [{"text":"impl&lt;A&gt; Ord for ArrayString&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array&lt;Item = u8&gt; + Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord&gt; Ord for CapacityError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Ord for ArrayVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: Ord,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["byteorder"] = [{"text":"impl Ord for BigEndian","synthetic":false,"types":[]},{"text":"impl Ord for LittleEndian","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl Ord for Bytes","synthetic":false,"types":[]},{"text":"impl Ord for BytesMut","synthetic":false,"types":[]}];
implementors["cpal"] = [{"text":"impl Ord for SampleRate","synthetic":false,"types":[]},{"text":"impl Ord for StreamInstant","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;'g, T&gt; Ord for Shared&lt;'g, T&gt;","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L:&nbsp;Ord, R:&nbsp;Ord&gt; Ord for Either&lt;L, R&gt;","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl&lt;T:&nbsp;Ord&gt; Ord for AllowStdIo&lt;T&gt;","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T:&nbsp;Ord, N&gt; Ord for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["gimli"] = [{"text":"impl Ord for Register","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord&gt; Ord for DebugInfoOffset&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord&gt; Ord for DebugTypesOffset&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord&gt; Ord for UnitSectionOffset&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Ord for SectionId","synthetic":false,"types":[]},{"text":"impl Ord for DwUt","synthetic":false,"types":[]},{"text":"impl Ord for DwCfa","synthetic":false,"types":[]},{"text":"impl Ord for DwChildren","synthetic":false,"types":[]},{"text":"impl Ord for DwTag","synthetic":false,"types":[]},{"text":"impl Ord for DwAt","synthetic":false,"types":[]},{"text":"impl Ord for DwForm","synthetic":false,"types":[]},{"text":"impl Ord for DwAte","synthetic":false,"types":[]},{"text":"impl Ord for DwLle","synthetic":false,"types":[]},{"text":"impl Ord for DwDs","synthetic":false,"types":[]},{"text":"impl Ord for DwEnd","synthetic":false,"types":[]},{"text":"impl Ord for DwAccess","synthetic":false,"types":[]},{"text":"impl Ord for DwVis","synthetic":false,"types":[]},{"text":"impl Ord for DwVirtuality","synthetic":false,"types":[]},{"text":"impl Ord for DwLang","synthetic":false,"types":[]},{"text":"impl Ord for DwAddr","synthetic":false,"types":[]},{"text":"impl Ord for DwId","synthetic":false,"types":[]},{"text":"impl Ord for DwCc","synthetic":false,"types":[]},{"text":"impl Ord for DwInl","synthetic":false,"types":[]},{"text":"impl Ord for DwOrd","synthetic":false,"types":[]},{"text":"impl Ord for DwDsc","synthetic":false,"types":[]},{"text":"impl Ord for DwIdx","synthetic":false,"types":[]},{"text":"impl Ord for DwDefaulted","synthetic":false,"types":[]},{"text":"impl Ord for DwLns","synthetic":false,"types":[]},{"text":"impl Ord for DwLne","synthetic":false,"types":[]},{"text":"impl Ord for DwLnct","synthetic":false,"types":[]},{"text":"impl Ord for DwMacro","synthetic":false,"types":[]},{"text":"impl Ord for DwRle","synthetic":false,"types":[]},{"text":"impl Ord for DwOp","synthetic":false,"types":[]},{"text":"impl Ord for DwEhPe","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy + Ord&gt; Ord for ArangeEntry&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Ord for ColumnType","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Ord&gt; Ord for UnitOffset&lt;T&gt;","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl Ord for HeaderValue","synthetic":false,"types":[]},{"text":"impl Ord for StatusCode","synthetic":false,"types":[]},{"text":"impl Ord for Version","synthetic":false,"types":[]}];
implementors["ipnet"] = [{"text":"impl Ord for IpAddrRange","synthetic":false,"types":[]},{"text":"impl Ord for Ipv4AddrRange","synthetic":false,"types":[]},{"text":"impl Ord for Ipv6AddrRange","synthetic":false,"types":[]},{"text":"impl Ord for IpNet","synthetic":false,"types":[]},{"text":"impl Ord for Ipv4Net","synthetic":false,"types":[]},{"text":"impl Ord for Ipv6Net","synthetic":false,"types":[]},{"text":"impl Ord for IpSubnets","synthetic":false,"types":[]},{"text":"impl Ord for Ipv4Subnets","synthetic":false,"types":[]},{"text":"impl Ord for Ipv6Subnets","synthetic":false,"types":[]}];
implementors["lexical_core"] = [{"text":"impl Ord for ErrorCode","synthetic":false,"types":[]},{"text":"impl Ord for Error","synthetic":false,"types":[]}];
implementors["log"] = [{"text":"impl Ord for Level","synthetic":false,"types":[]},{"text":"impl Ord for LevelFilter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Ord for Metadata&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Ord for MetadataBuilder&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["mime"] = [{"text":"impl&lt;'a&gt; Ord for Name&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Ord for Mime","synthetic":false,"types":[]}];
implementors["mio"] = [{"text":"impl Ord for PollOpt","synthetic":false,"types":[]},{"text":"impl Ord for Ready","synthetic":false,"types":[]},{"text":"impl Ord for UnixReady","synthetic":false,"types":[]},{"text":"impl Ord for Token","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl Ord for AtFlags","synthetic":false,"types":[]},{"text":"impl Ord for OFlag","synthetic":false,"types":[]},{"text":"impl Ord for SealFlag","synthetic":false,"types":[]},{"text":"impl Ord for FdFlag","synthetic":false,"types":[]},{"text":"impl Ord for SpliceFFlags","synthetic":false,"types":[]},{"text":"impl Ord for FallocateFlags","synthetic":false,"types":[]},{"text":"impl Ord for PosixFadviseAdvice","synthetic":false,"types":[]},{"text":"impl Ord for ModuleInitFlags","synthetic":false,"types":[]},{"text":"impl Ord for DeleteModuleFlags","synthetic":false,"types":[]},{"text":"impl Ord for MsFlags","synthetic":false,"types":[]},{"text":"impl Ord for MntFlags","synthetic":false,"types":[]},{"text":"impl Ord for MQ_OFlag","synthetic":false,"types":[]},{"text":"impl Ord for FdFlag","synthetic":false,"types":[]},{"text":"impl Ord for InterfaceFlags","synthetic":false,"types":[]},{"text":"impl Ord for PollFlags","synthetic":false,"types":[]},{"text":"impl Ord for CloneFlags","synthetic":false,"types":[]},{"text":"impl Ord for AioFsyncMode","synthetic":false,"types":[]},{"text":"impl Ord for LioOpcode","synthetic":false,"types":[]},{"text":"impl Ord for LioMode","synthetic":false,"types":[]},{"text":"impl Ord for EpollFlags","synthetic":false,"types":[]},{"text":"impl Ord for EpollCreateFlags","synthetic":false,"types":[]},{"text":"impl Ord for EfdFlags","synthetic":false,"types":[]},{"text":"impl Ord for MemFdCreateFlag","synthetic":false,"types":[]},{"text":"impl Ord for ProtFlags","synthetic":false,"types":[]},{"text":"impl Ord for MapFlags","synthetic":false,"types":[]},{"text":"impl Ord for MmapAdvise","synthetic":false,"types":[]},{"text":"impl Ord for MsFlags","synthetic":false,"types":[]},{"text":"impl Ord for MlockAllFlags","synthetic":false,"types":[]},{"text":"impl Ord for Request","synthetic":false,"types":[]},{"text":"impl Ord for Event","synthetic":false,"types":[]},{"text":"impl Ord for Options","synthetic":false,"types":[]},{"text":"impl Ord for QuotaType","synthetic":false,"types":[]},{"text":"impl Ord for QuotaFmt","synthetic":false,"types":[]},{"text":"impl Ord for QuotaValidFlags","synthetic":false,"types":[]},{"text":"impl Ord for RebootMode","synthetic":false,"types":[]},{"text":"impl Ord for Signal","synthetic":false,"types":[]},{"text":"impl Ord for SaFlags","synthetic":false,"types":[]},{"text":"impl Ord for SigmaskHow","synthetic":false,"types":[]},{"text":"impl Ord for SfdFlags","synthetic":false,"types":[]},{"text":"impl Ord for SockFlag","synthetic":false,"types":[]},{"text":"impl Ord for MsgFlags","synthetic":false,"types":[]},{"text":"impl Ord for SFlag","synthetic":false,"types":[]},{"text":"impl Ord for Mode","synthetic":false,"types":[]},{"text":"impl Ord for FsFlags","synthetic":false,"types":[]},{"text":"impl Ord for BaudRate","synthetic":false,"types":[]},{"text":"impl Ord for SetArg","synthetic":false,"types":[]},{"text":"impl Ord for FlushArg","synthetic":false,"types":[]},{"text":"impl Ord for FlowArg","synthetic":false,"types":[]},{"text":"impl Ord for SpecialCharacterIndices","synthetic":false,"types":[]},{"text":"impl Ord for InputFlags","synthetic":false,"types":[]},{"text":"impl Ord for OutputFlags","synthetic":false,"types":[]},{"text":"impl Ord for ControlFlags","synthetic":false,"types":[]},{"text":"impl Ord for LocalFlags","synthetic":false,"types":[]},{"text":"impl Ord for TimeSpec","synthetic":false,"types":[]},{"text":"impl Ord for TimeVal","synthetic":false,"types":[]},{"text":"impl Ord for WaitPidFlag","synthetic":false,"types":[]},{"text":"impl Ord for AddWatchFlags","synthetic":false,"types":[]},{"text":"impl Ord for InitFlags","synthetic":false,"types":[]},{"text":"impl Ord for WatchDescriptor","synthetic":false,"types":[]},{"text":"impl Ord for AccessFlags","synthetic":false,"types":[]}];
implementors["object"] = [{"text":"impl&lt;E:&nbsp;Ord + Endian&gt; Ord for U16Bytes&lt;E&gt;","synthetic":false,"types":[]},{"text":"impl&lt;E:&nbsp;Ord + Endian&gt; Ord for U32Bytes&lt;E&gt;","synthetic":false,"types":[]},{"text":"impl&lt;E:&nbsp;Ord + Endian&gt; Ord for U64Bytes&lt;E&gt;","synthetic":false,"types":[]},{"text":"impl&lt;E:&nbsp;Ord + Endian&gt; Ord for I16Bytes&lt;E&gt;","synthetic":false,"types":[]},{"text":"impl&lt;E:&nbsp;Ord + Endian&gt; Ord for I32Bytes&lt;E&gt;","synthetic":false,"types":[]},{"text":"impl&lt;E:&nbsp;Ord + Endian&gt; Ord for I64Bytes&lt;E&gt;","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl Ord for BigNumRef","synthetic":false,"types":[]},{"text":"impl Ord for BigNum","synthetic":false,"types":[]},{"text":"impl Ord for CMSOptions","synthetic":false,"types":[]},{"text":"impl Ord for OcspFlag","synthetic":false,"types":[]},{"text":"impl Ord for Pkcs7Flags","synthetic":false,"types":[]},{"text":"impl Ord for SslOptions","synthetic":false,"types":[]},{"text":"impl Ord for SslMode","synthetic":false,"types":[]},{"text":"impl Ord for SslVerifyMode","synthetic":false,"types":[]},{"text":"impl Ord for SslSessionCacheMode","synthetic":false,"types":[]},{"text":"impl Ord for ExtensionContext","synthetic":false,"types":[]},{"text":"impl Ord for ShutdownState","synthetic":false,"types":[]},{"text":"impl Ord for X509CheckFlags","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Ord for Ident","synthetic":false,"types":[]}];
implementors["signal_hook_registry"] = [{"text":"impl Ord for SigId","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl Ord for Lifetime","synthetic":false,"types":[]}];
implementors["time"] = [{"text":"impl Ord for Duration","synthetic":false,"types":[]},{"text":"impl Ord for Timespec","synthetic":false,"types":[]},{"text":"impl Ord for SteadyTime","synthetic":false,"types":[]},{"text":"impl Ord for Tm","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Ord for ArrayVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: Ord,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Ord for TinyVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: Ord,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tokio"] = [{"text":"impl Ord for Instant","synthetic":false,"types":[]}];
implementors["tokio_util"] = [{"text":"impl Ord for BytesCodec","synthetic":false,"types":[]},{"text":"impl Ord for LinesCodec","synthetic":false,"types":[]}];
implementors["tracing_core"] = [{"text":"impl Ord for Level","synthetic":false,"types":[]},{"text":"impl Ord for LevelFilter","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl Ord for B0","synthetic":false,"types":[]},{"text":"impl Ord for B1","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Ord + Unsigned + NonZero&gt; Ord for PInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Ord + Unsigned + NonZero&gt; Ord for NInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl Ord for Z0","synthetic":false,"types":[]},{"text":"impl Ord for UTerm","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Ord, B:&nbsp;Ord&gt; Ord for UInt&lt;U, B&gt;","synthetic":false,"types":[]},{"text":"impl Ord for ATerm","synthetic":false,"types":[]},{"text":"impl&lt;V:&nbsp;Ord, A:&nbsp;Ord&gt; Ord for TArr&lt;V, A&gt;","synthetic":false,"types":[]},{"text":"impl Ord for Greater","synthetic":false,"types":[]},{"text":"impl Ord for Less","synthetic":false,"types":[]},{"text":"impl Ord for Equal","synthetic":false,"types":[]}];
implementors["unicase"] = [{"text":"impl&lt;T:&nbsp;AsRef&lt;str&gt;&gt; Ord for Ascii&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;AsRef&lt;str&gt;&gt; Ord for UniCase&lt;T&gt;","synthetic":false,"types":[]}];
implementors["unicode_bidi"] = [{"text":"impl Ord for Level","synthetic":false,"types":[]}];
implementors["url"] = [{"text":"impl&lt;S:&nbsp;Ord&gt; Ord for Host&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl Ord for Url","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()