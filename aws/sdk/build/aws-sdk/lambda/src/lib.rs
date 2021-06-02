#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
//! <fullname>AWS Lambda</fullname>
//! <p>
//! <b>Overview</b>
//! </p>
//! <p>This is the <i>AWS Lambda API Reference</i>. The AWS Lambda Developer Guide provides additional
//! information. For the service overview, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/welcome.html">What is
//! AWS Lambda</a>, and for information about how the service works, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-introduction.html">AWS Lambda: How it Works</a> in the <b>AWS Lambda Developer Guide</b>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_json_errors;
mod blob_serde;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
mod http_serde;
pub mod input;
mod instant_epoch;
mod json_deser;
mod json_ser;
pub mod model;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
mod serde_util;
mod serializer;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("lambda", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
