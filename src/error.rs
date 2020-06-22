pub use error_chain::bail;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        ParseFloatError(::std::num::ParseFloatError);
        ParseIntError(::std::num::ParseIntError);
        ParseBoolError(::std::str::ParseBoolError);
        SerdeJson(serde_json::Error);
        RmpSerdeDecode(rmp_serde::decode::Error);
        Reqwest(reqwest::Error);
        RodioDecoder(rodio::decoder::DecoderError);
    }
}
