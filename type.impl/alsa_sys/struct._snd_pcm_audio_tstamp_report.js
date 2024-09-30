(function() {
    var type_impls = Object.fromEntries([["alsa_sys",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-_snd_pcm_audio_tstamp_report\" class=\"impl\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1185\">source</a><a href=\"#impl-Clone-for-_snd_pcm_audio_tstamp_report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html\" title=\"struct alsa_sys::_snd_pcm_audio_tstamp_report\">_snd_pcm_audio_tstamp_report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1185\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html\" title=\"struct alsa_sys::_snd_pcm_audio_tstamp_report\">_snd_pcm_audio_tstamp_report</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.6.0\">1.6.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","alsa_sys::snd_pcm_audio_tstamp_report_t"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-_snd_pcm_audio_tstamp_report\" class=\"impl\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1185\">source</a><a href=\"#impl-Debug-for-_snd_pcm_audio_tstamp_report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html\" title=\"struct alsa_sys::_snd_pcm_audio_tstamp_report\">_snd_pcm_audio_tstamp_report</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1185\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","alsa_sys::snd_pcm_audio_tstamp_report_t"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-_snd_pcm_audio_tstamp_report\" class=\"impl\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1190-1246\">source</a><a href=\"#impl-_snd_pcm_audio_tstamp_report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html\" title=\"struct alsa_sys::_snd_pcm_audio_tstamp_report\">_snd_pcm_audio_tstamp_report</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.valid\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1192-1194\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html#tymethod.valid\" class=\"fn\">valid</a>(&amp;self) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_uint.html\" title=\"type std::os::raw::c_uint\">c_uint</a></h4></section><section id=\"method.set_valid\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1196-1201\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html#tymethod.set_valid\" class=\"fn\">set_valid</a>(&amp;mut self, val: <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_uint.html\" title=\"type std::os::raw::c_uint\">c_uint</a>)</h4></section><section id=\"method.actual_type\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1203-1205\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html#tymethod.actual_type\" class=\"fn\">actual_type</a>(&amp;self) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_uint.html\" title=\"type std::os::raw::c_uint\">c_uint</a></h4></section><section id=\"method.set_actual_type\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1207-1212\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html#tymethod.set_actual_type\" class=\"fn\">set_actual_type</a>(&amp;mut self, val: <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_uint.html\" title=\"type std::os::raw::c_uint\">c_uint</a>)</h4></section><section id=\"method.accuracy_report\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1214-1216\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html#tymethod.accuracy_report\" class=\"fn\">accuracy_report</a>(&amp;self) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_uint.html\" title=\"type std::os::raw::c_uint\">c_uint</a></h4></section><section id=\"method.set_accuracy_report\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1218-1223\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html#tymethod.set_accuracy_report\" class=\"fn\">set_accuracy_report</a>(&amp;mut self, val: <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_uint.html\" title=\"type std::os::raw::c_uint\">c_uint</a>)</h4></section><section id=\"method.new_bitfield_1\" class=\"method\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1225-1245\">source</a><h4 class=\"code-header\">pub fn <a href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html#tymethod.new_bitfield_1\" class=\"fn\">new_bitfield_1</a>(\n    valid: <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_uint.html\" title=\"type std::os::raw::c_uint\">c_uint</a>,\n    actual_type: <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_uint.html\" title=\"type std::os::raw::c_uint\">c_uint</a>,\n    accuracy_report: <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/std/os/raw/type.c_uint.html\" title=\"type std::os::raw::c_uint\">c_uint</a>,\n) -&gt; <a class=\"struct\" href=\"alsa_sys/struct.__BindgenBitfieldUnit.html\" title=\"struct alsa_sys::__BindgenBitfieldUnit\">__BindgenBitfieldUnit</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">1</a>], <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;</h4></section></div></details>",0,"alsa_sys::snd_pcm_audio_tstamp_report_t"],["<section id=\"impl-Copy-for-_snd_pcm_audio_tstamp_report\" class=\"impl\"><a class=\"src rightside\" href=\"src/alsa_sys/generated.rs.html#1185\">source</a><a href=\"#impl-Copy-for-_snd_pcm_audio_tstamp_report\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"alsa_sys/struct._snd_pcm_audio_tstamp_report.html\" title=\"struct alsa_sys::_snd_pcm_audio_tstamp_report\">_snd_pcm_audio_tstamp_report</a></h3></section>","Copy","alsa_sys::snd_pcm_audio_tstamp_report_t"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[9162]}