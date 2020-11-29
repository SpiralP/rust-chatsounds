(function() {var implementors = {};
implementors["alsa"] = [{"text":"impl Iterator for Iter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for ElemIter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for Iter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Iterator for HintIter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for Iter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for IterEnum&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for ClientIter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for PortIter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for PortSubscribeIter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Iterator for ChmapsQuery","synthetic":false,"types":[]},{"text":"impl&lt;'a, S:&nbsp;'static + Copy&gt; Iterator for CaptureIter&lt;'a, S&gt;","synthetic":false,"types":[]}];
implementors["arrayvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Iterator for IntoIter&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, A:&nbsp;Array&gt; Iterator for Drain&lt;'a, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: 'a,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["bitvec"] = [{"text":"impl&lt;'a, T&gt; Iterator for Domain&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for Iter&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for IterMut&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for Windows&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for Chunks&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for ChunksMut&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for ChunksExact&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for ChunksExactMut&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for RChunks&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for RChunksMut&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for RChunksExact&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T&gt; Iterator for RChunksExactMut&lt;'a, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; Iterator for Split&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; Iterator for SplitMut&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; Iterator for RSplit&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; Iterator for RSplitMut&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; Iterator for SplitN&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; Iterator for SplitNMut&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; Iterator for RSplitN&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; Iterator for RSplitNMut&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; Iterator for IntoIter&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, '_&gt; Iterator for Drain&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, I, '_&gt; Iterator for Splice&lt;'_, O, T, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator&lt;Item = bool&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl&lt;T:&nbsp;Buf&gt; Iterator for IntoIter&lt;T&gt;","synthetic":false,"types":[]}];
implementors["claxon"] = [{"text":"impl&lt;'a&gt; Iterator for StereoSamples&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for Tags&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for GetTag&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;ReadBytes&gt; Iterator for MetadataBlockReader&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;ReadBytes&gt; Iterator for FlacSamples&lt;R&gt;","synthetic":false,"types":[]}];
implementors["cpal"] = [{"text":"impl Iterator for Devices","synthetic":false,"types":[]},{"text":"impl Iterator for Devices","synthetic":false,"types":[]},{"text":"impl Iterator for SupportedInputConfigs","synthetic":false,"types":[]},{"text":"impl Iterator for SupportedOutputConfigs","synthetic":false,"types":[]}];
implementors["crossbeam_channel"] = [{"text":"impl&lt;T, '_&gt; Iterator for Iter&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, '_&gt; Iterator for TryIter&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Iterator for IntoIter&lt;T&gt;","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; Iterator for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Iterator&lt;Item = L::Item&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["error_chain"] = [{"text":"impl&lt;'a&gt; Iterator for Iter&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["form_urlencoded"] = [{"text":"impl&lt;'a&gt; Iterator for Parse&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for ParseIntoOwned&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for ByteSerialize&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["futures_executor"] = [{"text":"impl&lt;S:&nbsp;Stream + Unpin&gt; Iterator for BlockingStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl&lt;'a, Fut&gt; Iterator for IterPinMut&lt;'a, Fut&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, Fut:&nbsp;Unpin&gt; Iterator for IterMut&lt;'a, Fut&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, Fut&gt; Iterator for IterPinRef&lt;'a, Fut&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, Fut:&nbsp;Unpin&gt; Iterator for Iter&lt;'a, Fut&gt;","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; Iterator for GenericArrayIter&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["gimli"] = [{"text":"impl&lt;'iter, R:&nbsp;Reader&gt; Iterator for RegisterRuleIter&lt;'iter, R&gt;","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;T&gt; Iterator for RawIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Iterator for RawIntoIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, '_&gt; Iterator for RawDrain&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for RawIterHash&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V, F, '_&gt; Iterator for DrainFilter&lt;'_, K, V, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnMut(&amp;K, &amp;mut V) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for Iter&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for IterMut&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; Iterator for IntoIter&lt;K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for Keys&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for Values&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for ValuesMut&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for Drain&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K&gt; Iterator for Iter&lt;'a, K&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K&gt; Iterator for IntoIter&lt;K&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, '_&gt; Iterator for Drain&lt;'_, K&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, F, '_&gt; Iterator for DrainFilter&lt;'_, K, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnMut(&amp;K) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Iterator for Intersection&lt;'a, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Iterator for Difference&lt;'a, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Iterator for SymmetricDifference&lt;'a, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Iterator for Union&lt;'a, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["hound"] = [{"text":"impl&lt;'wr, R, S&gt; Iterator for WavSamples&lt;'wr, R, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Read,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;R, S&gt; Iterator for WavIntoSamples&lt;R, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Read,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sample,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl&lt;'a, T&gt; Iterator for Iter&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for IterMut&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for Keys&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for Values&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for ValuesMut&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for Drain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;'a&gt; Iterator for ValueIter&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;'a&gt; Iterator for ValueIterMut&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Iterator for IntoIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for ValueDrain&lt;'a, T&gt;","synthetic":false,"types":[]}];
implementors["hyper"] = [{"text":"impl Iterator for GaiAddrs","synthetic":false,"types":[]}];
implementors["indexmap"] = [{"text":"impl&lt;'a, K, V&gt; Iterator for Keys&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for Values&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for ValuesMut&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for Iter&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; Iterator for IterMut&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; Iterator for IntoIter&lt;K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V, '_&gt; Iterator for Drain&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Iterator for IntoIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for Iter&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, '_&gt; Iterator for Drain&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Iterator for Difference&lt;'a, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Iterator for Intersection&lt;'a, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S1, S2&gt; Iterator for SymmetricDifference&lt;'a, T, S1, S2&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S1: BuildHasher,<br>&nbsp;&nbsp;&nbsp;&nbsp;S2: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Iterator for Union&lt;'a, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["ipnet"] = [{"text":"impl Iterator for IpAddrRange","synthetic":false,"types":[]},{"text":"impl Iterator for Ipv4AddrRange","synthetic":false,"types":[]},{"text":"impl Iterator for Ipv6AddrRange","synthetic":false,"types":[]},{"text":"impl Iterator for IpSubnets","synthetic":false,"types":[]},{"text":"impl Iterator for Ipv4Subnets","synthetic":false,"types":[]},{"text":"impl Iterator for Ipv6Subnets","synthetic":false,"types":[]}];
implementors["memchr"] = [{"text":"impl&lt;'a&gt; Iterator for Memchr&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for Memchr2&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for Memchr3&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["mime"] = [{"text":"impl&lt;'a&gt; Iterator for Params&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["mime_guess"] = [{"text":"impl Iterator for Iter","synthetic":false,"types":[]},{"text":"impl Iterator for IterRaw","synthetic":false,"types":[]}];
implementors["mio"] = [{"text":"impl&lt;'a&gt; Iterator for Iter&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl&lt;'d&gt; Iterator for Iter&lt;'d&gt;","synthetic":false,"types":[]},{"text":"impl Iterator for InterfaceAddressIterator","synthetic":false,"types":[]},{"text":"impl Iterator for SignalIterator","synthetic":false,"types":[]},{"text":"impl Iterator for SignalFd","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for CmsgIterator&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["object"] = [{"text":"impl&lt;'data, 'file&gt; Iterator for SegmentIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for SectionIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for ComdatIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for ComdatSectionIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for SymbolIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for RelocationIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data&gt; Iterator for ArchiveMemberIterator&lt;'data&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for CoffSegmentIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for CoffSectionIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for CoffSymbolIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for CoffRelocationIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for CoffComdatIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for CoffComdatSectionIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Elf:&nbsp;FileHeader&gt; Iterator for ElfSegmentIterator&lt;'data, 'file, Elf&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Elf:&nbsp;FileHeader&gt; Iterator for ElfSectionIterator&lt;'data, 'file, Elf&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Elf:&nbsp;FileHeader&gt; Iterator for ElfSymbolIterator&lt;'data, 'file, Elf&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Elf:&nbsp;FileHeader&gt; Iterator for ElfRelocationIterator&lt;'data, 'file, Elf&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Elf:&nbsp;FileHeader&gt; Iterator for ElfComdatIterator&lt;'data, 'file, Elf&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Elf:&nbsp;FileHeader&gt; Iterator for ElfComdatSectionIterator&lt;'data, 'file, Elf&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Mach:&nbsp;MachHeader&gt; Iterator for MachOComdatIterator&lt;'data, 'file, Mach&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Mach:&nbsp;MachHeader&gt; Iterator for MachOComdatSectionIterator&lt;'data, 'file, Mach&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Mach:&nbsp;MachHeader&gt; Iterator for MachOSegmentIterator&lt;'data, 'file, Mach&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Mach:&nbsp;MachHeader&gt; Iterator for MachOSectionIterator&lt;'data, 'file, Mach&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Mach:&nbsp;MachHeader&gt; Iterator for MachOSymbolIterator&lt;'data, 'file, Mach&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Mach:&nbsp;MachHeader&gt; Iterator for MachORelocationIterator&lt;'data, 'file, Mach&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Pe:&nbsp;ImageNtHeaders&gt; Iterator for PeComdatIterator&lt;'data, 'file, Pe&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Pe:&nbsp;ImageNtHeaders&gt; Iterator for PeComdatSectionIterator&lt;'data, 'file, Pe&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Pe:&nbsp;ImageNtHeaders&gt; Iterator for PeSegmentIterator&lt;'data, 'file, Pe&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file, Pe:&nbsp;ImageNtHeaders&gt; Iterator for PeSectionIterator&lt;'data, 'file, Pe&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'data, 'file&gt; Iterator for PeRelocationIterator&lt;'data, 'file&gt;","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl&lt;T:&nbsp;Stackable&gt; Iterator for IntoIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Stackable&gt; Iterator for Iter&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Stackable&gt; Iterator for IterMut&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for X509NameEntries&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["percent_encoding"] = [{"text":"impl&lt;'a&gt; Iterator for PercentEncode&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for PercentDecode&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Iterator for IntoIter","synthetic":false,"types":[]}];
implementors["rand"] = [{"text":"impl&lt;D, R, T&gt; Iterator for DistIter&lt;D, R, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Distribution&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Rng,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for IndexVecIter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Iterator for IndexVecIntoIter","synthetic":false,"types":[]},{"text":"impl&lt;'a, S:&nbsp;Index&lt;usize, Output = T&gt; + ?Sized + 'a, T:&nbsp;'a&gt; Iterator for SliceChooseIter&lt;'a, S, T&gt;","synthetic":false,"types":[]}];
implementors["rodio"] = [{"text":"impl&lt;S&gt; Iterator for SamplesBuffer&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; Iterator for Decoder&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Read + Seek,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;R&gt; Iterator for LoopedDecoder&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Read + Seek,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S&gt; Iterator for DynamicMixer&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sample + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S&gt; Iterator for SourcesQueueOutput&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sample + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for Amplify&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for BltFilter&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source&lt;Item = f32&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for Buffered&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for ChannelVolume&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for Delay&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Source&gt; Iterator for Done&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S&gt; Iterator for Empty&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for FadeIn&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;F, S&gt; Iterator for FromFactoryIter&lt;F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnMut() -&gt; Option&lt;S&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for FromIter&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Iterator + Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;I::Item as Iterator&gt;::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I1, I2&gt; Iterator for Mix&lt;I1, I2&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I1: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I1::Item: Sample,<br>&nbsp;&nbsp;&nbsp;&nbsp;I2: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I2::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for Pausable&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I, F&gt; Iterator for PeriodicAccess&lt;I, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnMut(&amp;mut I),&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for Repeat&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I, D&gt; Iterator for SamplesConverter&lt;I, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Iterator for SineWave","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for SkipDuration&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for Spatial&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample + Debug,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for Speed&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for Stoppable&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I&gt; Iterator for TakeDuration&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;I, D&gt; Iterator for UniformSourceIterator&lt;I, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Source,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: Sample,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S&gt; Iterator for Zero&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sample,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S&gt; Iterator for StaticSamplesBuffer&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sample + Clone,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl&lt;'de, R, T&gt; Iterator for StreamDeserializer&lt;'de, R, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Read&lt;'de&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Deserialize&lt;'de&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for Iter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for IterMut&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Iterator for IntoIter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for Keys&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for Values&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Iterator for ValuesMut&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["slab"] = [{"text":"impl&lt;'a, T&gt; Iterator for Iter&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for IterMut&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for Drain&lt;'a, T&gt;","synthetic":false,"types":[]}];
implementors["slice_deque"] = [{"text":"impl&lt;'a, T&gt; Iterator for Drain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Iterator for IntoIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, I:&nbsp;Iterator&gt; Iterator for Splice&lt;'a, I&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, F&gt; Iterator for DrainFilter&lt;'a, T, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnMut(&amp;mut T) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;'a, T:&nbsp;'a + Array&gt; Iterator for Drain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Iterator for IntoIter&lt;A&gt;","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;'a, T, P&gt; Iterator for Pairs&lt;'a, T, P&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, P&gt; Iterator for PairsMut&lt;'a, T, P&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, P&gt; Iterator for IntoPairs&lt;T, P&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Iterator for IntoIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for Iter&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Iterator for IterMut&lt;'a, T&gt;","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;'p, A:&nbsp;Array, I:&nbsp;Iterator&lt;Item = A::Item&gt;&gt; Iterator for ArrayVecSplice&lt;'p, A, I&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Iterator for ArrayVecIterator&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;'a + Default&gt; Iterator for ArrayVecDrain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p, 's, T:&nbsp;Default&gt; Iterator for SliceVecDrain&lt;'p, 's, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p, A:&nbsp;Array&gt; Iterator for TinyVecDrain&lt;'p, A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p, A, I&gt; Iterator for TinyVecSplice&lt;'p, A, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator&lt;Item = A::Item&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Iterator for TinyVecIterator&lt;A&gt;","synthetic":false,"types":[]}];
implementors["tracing_core"] = [{"text":"impl Iterator for Iter","synthetic":false,"types":[]}];
implementors["unicode_normalization"] = [{"text":"impl&lt;I:&nbsp;Iterator&lt;Item = char&gt;&gt; Iterator for Decompositions&lt;I&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Iterator&lt;Item = char&gt;&gt; Iterator for Recompositions&lt;I&gt;","synthetic":false,"types":[]},{"text":"impl&lt;I:&nbsp;Iterator&lt;Item = char&gt;&gt; Iterator for StreamSafe&lt;I&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()