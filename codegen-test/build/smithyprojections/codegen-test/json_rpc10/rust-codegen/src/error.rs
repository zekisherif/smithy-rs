// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use crate::model::ComplexNestedErrorData;
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum EmptyInputAndEmptyOutputError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for EmptyInputAndEmptyOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmptyInputAndEmptyOutputError::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl EmptyInputAndEmptyOutputError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        EmptyInputAndEmptyOutputError::Unhandled(err.into())
    }
}
impl ::std::error::Error for EmptyInputAndEmptyOutputError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            EmptyInputAndEmptyOutputError::Unhandled(inner) => Some(inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum GreetingWithErrorsError {
    InvalidGreeting(InvalidGreeting),
    ComplexError(ComplexError),
    FooError(FooError),

    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for GreetingWithErrorsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreetingWithErrorsError::InvalidGreeting(inner) => inner.fmt(f),
            GreetingWithErrorsError::ComplexError(inner) => inner.fmt(f),
            GreetingWithErrorsError::FooError(inner) => inner.fmt(f),
            GreetingWithErrorsError::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl GreetingWithErrorsError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        GreetingWithErrorsError::Unhandled(err.into())
    }
}
impl ::std::error::Error for GreetingWithErrorsError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            GreetingWithErrorsError::InvalidGreeting(inner) => Some(inner),
            GreetingWithErrorsError::ComplexError(inner) => Some(inner),
            GreetingWithErrorsError::FooError(inner) => Some(inner),
            GreetingWithErrorsError::Unhandled(inner) => Some(inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum JsonUnionsError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for JsonUnionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonUnionsError::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl JsonUnionsError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        JsonUnionsError::Unhandled(err.into())
    }
}
impl ::std::error::Error for JsonUnionsError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            JsonUnionsError::Unhandled(inner) => Some(inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum NoInputAndNoOutputError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for NoInputAndNoOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoInputAndNoOutputError::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl NoInputAndNoOutputError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        NoInputAndNoOutputError::Unhandled(err.into())
    }
}
impl ::std::error::Error for NoInputAndNoOutputError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            NoInputAndNoOutputError::Unhandled(inner) => Some(inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum NoInputAndOutputError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for NoInputAndOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoInputAndOutputError::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl NoInputAndOutputError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        NoInputAndOutputError::Unhandled(err.into())
    }
}
impl ::std::error::Error for NoInputAndOutputError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            NoInputAndOutputError::Unhandled(inner) => Some(inner.as_ref()),
        }
    }
}

/// This error has test cases that test some of the dark corners of Amazon service
/// framework history. It should only be implemented by clients.
#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct FooError {}
impl FooError {
    pub fn retryable(&self) -> bool {
        false
    }
    pub fn throttling(&self) -> bool {
        false
    }
    pub fn code(&self) -> &str {
        "FooError"
    }
    pub fn message(&self) -> Option<&str> {
        None
    }
}
impl ::std::fmt::Display for FooError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FooError")?;
        Ok(())
    }
}
impl ::std::error::Error for FooError {}
/// See [`FooError`](crate::error::FooError)
pub mod foo_error {

    use crate::error::FooError;
    /// A builder for [`FooError`](crate::error::FooError)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`FooError`](crate::error::FooError)
        pub fn build(self) -> FooError {
            FooError {}
        }
    }
}
impl FooError {
    /// Creates a new builder-style object to manufacture [`FooError`](crate::error::FooError)
    pub fn builder() -> crate::error::foo_error::Builder {
        crate::error::foo_error::Builder::default()
    }
}

/// This error is thrown when a request is invalid.
#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct ComplexError {
    #[serde(rename = "TopLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top_level: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Nested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub nested: ::std::option::Option<ComplexNestedErrorData>,
}
impl ComplexError {
    pub fn retryable(&self) -> bool {
        false
    }
    pub fn throttling(&self) -> bool {
        false
    }
    pub fn code(&self) -> &str {
        "ComplexError"
    }
    pub fn message(&self) -> Option<&str> {
        None
    }
}
impl ::std::fmt::Display for ComplexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ComplexError")?;
        Ok(())
    }
}
impl ::std::error::Error for ComplexError {}
/// See [`ComplexError`](crate::error::ComplexError)
pub mod complex_error {

    use crate::error::ComplexError;
    use crate::model::ComplexNestedErrorData;
    /// A builder for [`ComplexError`](crate::error::ComplexError)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        top_level: ::std::option::Option<::std::string::String>,
        nested: ::std::option::Option<ComplexNestedErrorData>,
    }
    impl Builder {
        pub fn top_level(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.top_level = Some(inp.into());
            self
        }
        pub fn nested(mut self, inp: ComplexNestedErrorData) -> Self {
            self.nested = Some(inp);
            self
        }
        /// Consumes the builder and constructs a [`ComplexError`](crate::error::ComplexError)
        pub fn build(self) -> ComplexError {
            ComplexError {
                top_level: self.top_level,
                nested: self.nested,
            }
        }
    }
}
impl ComplexError {
    /// Creates a new builder-style object to manufacture [`ComplexError`](crate::error::ComplexError)
    pub fn builder() -> crate::error::complex_error::Builder {
        crate::error::complex_error::Builder::default()
    }
}

/// This error is thrown when an invalid greeting value is provided.
#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct InvalidGreeting {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: ::std::option::Option<::std::string::String>,
}
impl InvalidGreeting {
    pub fn retryable(&self) -> bool {
        false
    }
    pub fn throttling(&self) -> bool {
        false
    }
    pub fn code(&self) -> &str {
        "InvalidGreeting"
    }
    pub fn message(&self) -> Option<&str> {
        None
    }
}
impl ::std::fmt::Display for InvalidGreeting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidGreeting")?;
        Ok(())
    }
}
impl ::std::error::Error for InvalidGreeting {}
/// See [`InvalidGreeting`](crate::error::InvalidGreeting)
pub mod invalid_greeting {

    use crate::error::InvalidGreeting;
    /// A builder for [`InvalidGreeting`](crate::error::InvalidGreeting)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        message: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.message = Some(inp.into());
            self
        }
        /// Consumes the builder and constructs a [`InvalidGreeting`](crate::error::InvalidGreeting)
        pub fn build(self) -> InvalidGreeting {
            InvalidGreeting {
                message: self.message,
            }
        }
    }
}
impl InvalidGreeting {
    /// Creates a new builder-style object to manufacture [`InvalidGreeting`](crate::error::InvalidGreeting)
    pub fn builder() -> crate::error::invalid_greeting::Builder {
        crate::error::invalid_greeting::Builder::default()
    }
}
