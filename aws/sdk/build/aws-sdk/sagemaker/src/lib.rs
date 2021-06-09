#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
//! <p>Provides APIs for creating and managing Amazon SageMaker resources. </p>
//! <p>Other Resources:</p>
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/whatis.html#first-time-user">Amazon SageMaker Developer
//! Guide</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/augmented-ai/2019-11-07/APIReference/Welcome.html">Amazon Augmented AI
//! Runtime API Reference</a>
//! </p>
//! </li>
//! </ul>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
mod aws_json_errors;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
mod idempotency_token;
pub mod input;
mod instant_epoch;
mod json_deser;
mod json_ser;
pub mod model;
pub mod operation;
mod operation_ser;
pub mod output;
mod serde_util;
mod serializer;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("sagemaker", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
