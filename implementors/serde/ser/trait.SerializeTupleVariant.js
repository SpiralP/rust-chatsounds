(function() {var implementors = {};
implementors["rmp_serde"] = [{"text":"impl&lt;'a, W:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> + 'a, C:&nbsp;<a class=\"trait\" href=\"rmp_serde/config/trait.SerializerConfig.html\" title=\"trait rmp_serde::config::SerializerConfig\">SerializerConfig</a>&gt; <a class=\"trait\" href=\"serde/ser/trait.SerializeTupleVariant.html\" title=\"trait serde::ser::SerializeTupleVariant\">SerializeTupleVariant</a> for <a class=\"struct\" href=\"rmp_serde/encode/struct.Compound.html\" title=\"struct rmp_serde::encode::Compound\">Compound</a>&lt;'a, W, C&gt;","synthetic":false,"types":["rmp_serde::encode::Compound"]}];
implementors["serde"] = [];
implementors["serde_urlencoded"] = [{"text":"impl&lt;'input, 'output, Target&gt; <a class=\"trait\" href=\"serde/ser/trait.SerializeTupleVariant.html\" title=\"trait serde::ser::SerializeTupleVariant\">SerializeTupleVariant</a> for <a class=\"struct\" href=\"serde_urlencoded/ser/struct.TupleVariantSerializer.html\" title=\"struct serde_urlencoded::ser::TupleVariantSerializer\">TupleVariantSerializer</a>&lt;'input, 'output, Target&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Target: 'output + <a class=\"trait\" href=\"url/form_urlencoded/trait.Target.html\" title=\"trait url::form_urlencoded::Target\">UrlEncodedTarget</a>,&nbsp;</span>","synthetic":false,"types":["serde_urlencoded::ser::TupleVariantSerializer"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()