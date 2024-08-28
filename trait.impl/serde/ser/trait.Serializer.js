(function() {
    var implementors = Object.fromEntries([["rmp_serde",[["impl&lt;'a, W, C&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut <a class=\"struct\" href=\"rmp_serde/encode/struct.Serializer.html\" title=\"struct rmp_serde::encode::Serializer\">Serializer</a>&lt;W, C&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,\n    C: <a class=\"trait\" href=\"rmp_serde/config/trait.SerializerConfig.html\" title=\"trait rmp_serde::config::SerializerConfig\">SerializerConfig</a>,</div>"],["impl&lt;'a, W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> + 'a&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;mut <a class=\"struct\" href=\"rmp_serde/encode/struct.ExtFieldSerializer.html\" title=\"struct rmp_serde::encode::ExtFieldSerializer\">ExtFieldSerializer</a>&lt;'a, W&gt;"],["impl&lt;'a, W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> + 'a&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;mut <a class=\"struct\" href=\"rmp_serde/encode/struct.ExtSerializer.html\" title=\"struct rmp_serde::encode::ExtSerializer\">ExtSerializer</a>&lt;'a, W&gt;"]]],["serde",[]],["serde_json",[["impl <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for <a class=\"struct\" href=\"serde_json/value/struct.Serializer.html\" title=\"struct serde_json::value::Serializer\">Serializer</a>"],["impl&lt;'a, W, F&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut <a class=\"struct\" href=\"serde_json/struct.Serializer.html\" title=\"struct serde_json::Serializer\">Serializer</a>&lt;W, F&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,\n    F: <a class=\"trait\" href=\"serde_json/ser/trait.Formatter.html\" title=\"trait serde_json::ser::Formatter\">Formatter</a>,</div>"]]],["serde_urlencoded",[["impl&lt;'input, 'output, Target&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for <a class=\"struct\" href=\"serde_urlencoded/struct.Serializer.html\" title=\"struct serde_urlencoded::Serializer\">Serializer</a>&lt;'input, 'output, Target&gt;<div class=\"where\">where\n    Target: 'output + <a class=\"trait\" href=\"form_urlencoded/trait.Target.html\" title=\"trait form_urlencoded::Target\">UrlEncodedTarget</a>,</div>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[1521,13,858,521]}