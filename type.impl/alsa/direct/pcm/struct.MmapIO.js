(function() {
    var type_impls = Object.fromEntries([["alsa",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-MmapIO%3CS,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#308\">source</a><a href=\"#impl-Debug-for-MmapIO%3CS,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, D: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"alsa/direct/pcm/struct.MmapIO.html\" title=\"struct alsa::direct::pcm::MmapIO\">MmapIO</a>&lt;S, D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#308\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","alsa::direct::pcm::MmapPlayback","alsa::direct::pcm::MmapCapture"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-MmapIO%3CS,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#366-436\">source</a><a href=\"#impl-MmapIO%3CS,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S, D: <a class=\"trait\" href=\"alsa/direct/pcm/trait.MmapDir.html\" title=\"trait alsa::direct::pcm::MmapDir\">MmapDir</a>&gt; <a class=\"struct\" href=\"alsa/direct/pcm/struct.MmapIO.html\" title=\"struct alsa::direct::pcm::MmapIO\">MmapIO</a>&lt;S, D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.status\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#368\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa/direct/pcm/struct.MmapIO.html#tymethod.status\" class=\"fn\">status</a>(&amp;self) -&gt; &amp;<a class=\"struct\" href=\"alsa/direct/pcm/struct.Status.html\" title=\"struct alsa::direct::pcm::Status\">Status</a></h4></section></summary><div class=\"docblock\"><p>Read current status</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.appl_ptr\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#374\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa/direct/pcm/struct.MmapIO.html#tymethod.appl_ptr\" class=\"fn\">appl_ptr</a>(&amp;self) -&gt; <a class=\"type\" href=\"alsa/pcm/type.Frames.html\" title=\"type alsa::pcm::Frames\">Frames</a></h4></section></summary><div class=\"docblock\"><p>Read current number of frames committed by application</p>\n<p>This number wraps at ‘boundary’.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hw_ptr\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#380\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa/direct/pcm/struct.MmapIO.html#tymethod.hw_ptr\" class=\"fn\">hw_ptr</a>(&amp;self) -&gt; <a class=\"type\" href=\"alsa/pcm/type.Frames.html\" title=\"type alsa::pcm::Frames\">Frames</a></h4></section></summary><div class=\"docblock\"><p>Read current number of frames read / written by hardware</p>\n<p>This number wraps at ‘boundary’.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.boundary\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#384\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa/direct/pcm/struct.MmapIO.html#tymethod.boundary\" class=\"fn\">boundary</a>(&amp;self) -&gt; <a class=\"type\" href=\"alsa/pcm/type.Frames.html\" title=\"type alsa::pcm::Frames\">Frames</a></h4></section></summary><div class=\"docblock\"><p>The number at which hw_ptr and appl_ptr wraps.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.buffer_size\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#388\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa/direct/pcm/struct.MmapIO.html#tymethod.buffer_size\" class=\"fn\">buffer_size</a>(&amp;self) -&gt; <a class=\"type\" href=\"alsa/pcm/type.Frames.html\" title=\"type alsa::pcm::Frames\">Frames</a></h4></section></summary><div class=\"docblock\"><p>Total number of frames in hardware buffer</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.channels\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#392\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa/direct/pcm/struct.MmapIO.html#tymethod.channels\" class=\"fn\">channels</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a></h4></section></summary><div class=\"docblock\"><p>Number of channels in stream</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.commit\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#397-401\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa/direct/pcm/struct.MmapIO.html#tymethod.commit\" class=\"fn\">commit</a>(&amp;self, v: <a class=\"type\" href=\"alsa/pcm/type.Frames.html\" title=\"type alsa::pcm::Frames\">Frames</a>)</h4></section></summary><div class=\"docblock\"><p>Notifies the kernel that frames have now been read / written by the application</p>\n<p>This will allow the kernel to write new data into this part of the buffer.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.avail\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#406\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa/direct/pcm/struct.MmapIO.html#tymethod.avail\" class=\"fn\">avail</a>(&amp;self) -&gt; <a class=\"type\" href=\"alsa/pcm/type.Frames.html\" title=\"type alsa::pcm::Frames\">Frames</a></h4></section></summary><div class=\"docblock\"><p>Number of frames available to read / write.</p>\n<p>In case of an underrun, this value might be bigger than the buffer size.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.data_ptr\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa/direct/pcm.rs.html#416-435\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa/direct/pcm/struct.MmapIO.html#tymethod.data_ptr\" class=\"fn\">data_ptr</a>(&amp;self) -&gt; (<a class=\"struct\" href=\"alsa/direct/pcm/struct.RawSamples.html\" title=\"struct alsa::direct::pcm::RawSamples\">RawSamples</a>&lt;S&gt;, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"alsa/direct/pcm/struct.RawSamples.html\" title=\"struct alsa::direct::pcm::RawSamples\">RawSamples</a>&lt;S&gt;&gt;)</h4></section></summary><div class=\"docblock\"><p>Returns raw pointers to data to read / write.</p>\n<p>Use this if you want to read/write data yourself (instead of using iterators). If you do,\nusing <code>write_volatile</code> or <code>read_volatile</code> is recommended, since it’s DMA memory and can\nchange at any time.</p>\n<p>Since this is a ring buffer, there might be more data to read/write in the beginning\nof the buffer as well. If so this is returned as the second return value.</p>\n</div></details></div></details>",0,"alsa::direct::pcm::MmapPlayback","alsa::direct::pcm::MmapCapture"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[8408]}