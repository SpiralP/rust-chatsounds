(function() {var implementors = {};
implementors["rmp_serde"] = [{"text":"impl&lt;'a, W, C&gt; Serializer for &amp;'a mut Serializer&lt;W, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;W: Write,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: SerializerConfig,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, W:&nbsp;Write + 'a, '_&gt; Serializer for &amp;'_ mut ExtFieldSerializer&lt;'a, W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, W:&nbsp;Write + 'a, '_&gt; Serializer for &amp;'_ mut ExtSerializer&lt;'a, W&gt;","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl&lt;'a, W, F&gt; Serializer for &amp;'a mut Serializer&lt;W, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;W: Write,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Formatter,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Serializer for Serializer","synthetic":false,"types":[]}];
implementors["serde_urlencoded"] = [{"text":"impl&lt;'input, 'output, Target&gt; Serializer for Serializer&lt;'input, 'output, Target&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Target: 'output + UrlEncodedTarget,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()