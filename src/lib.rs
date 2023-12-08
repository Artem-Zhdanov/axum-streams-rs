#![allow(unused_parens, clippy::new_without_default)]
#![forbid(unsafe_code)]
#![allow(clippy::new_without_default, clippy::needless_lifetimes)]

mod stream_format;

mod stream_body_as;
pub use self::stream_body_as::StreamBodyAs;

#[cfg(feature = "json")]
mod json_formats;
#[cfg(feature = "json")]
pub use json_formats::JsonArrayStreamFormat;
#[cfg(feature = "json")]
pub use json_formats::JsonNewLineStreamFormat;

#[cfg(feature = "csv")]
mod csv_format;
#[cfg(feature = "csv")]
pub use csv::{QuoteStyle, Terminator};
#[cfg(feature = "csv")]
pub use csv_format::CsvStreamFormat;

#[cfg(feature = "text")]
mod text_format;
#[cfg(feature = "text")]
pub use text_format::TextStreamFormat;

#[cfg(feature = "protobuf")]
mod protobuf_format;
#[cfg(feature = "protobuf")]
pub use protobuf_format::ProtobufStreamFormat;

#[cfg(test)]
mod test_client;
