// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    InternalFailure(crate::error::InternalFailure),
    ModelError(crate::error::ModelError),
    ServiceUnavailable(crate::error::ServiceUnavailable),
    ValidationError(crate::error::ValidationError),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalFailure(inner) => inner.fmt(f),
            Error::ModelError(inner) => inner.fmt(f),
            Error::ServiceUnavailable(inner) => inner.fmt(f),
            Error::ValidationError(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::InvokeEndpointError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::InvokeEndpointError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::InvokeEndpointErrorKind::InternalFailure(inner) => {
                    Error::InternalFailure(inner)
                }
                crate::error::InvokeEndpointErrorKind::ModelError(inner) => {
                    Error::ModelError(inner)
                }
                crate::error::InvokeEndpointErrorKind::ServiceUnavailable(inner) => {
                    Error::ServiceUnavailable(inner)
                }
                crate::error::InvokeEndpointErrorKind::ValidationError(inner) => {
                    Error::ValidationError(inner)
                }
                crate::error::InvokeEndpointErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
