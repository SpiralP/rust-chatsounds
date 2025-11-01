(function() {
    var type_impls = Object.fromEntries([["alsa_sys",[]],["openssl_sys",[]],["serde",[]],["serde_core",[]],["tokio",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[15,19,13,18,13]}