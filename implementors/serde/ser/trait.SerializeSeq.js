(function() {var implementors = {};
implementors["rmp_serde"] = [{"text":"impl&lt;'a, W:&nbsp;Write + 'a, C:&nbsp;SerializerConfig&gt; SerializeSeq for Compound&lt;'a, W, C&gt;","synthetic":false,"types":[]}];
implementors["serde"] = [];
implementors["serde_urlencoded"] = [{"text":"impl&lt;'input, 'output, Target&gt; SerializeSeq for SeqSerializer&lt;'input, 'output, Target&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Target: 'output + UrlEncodedTarget,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()