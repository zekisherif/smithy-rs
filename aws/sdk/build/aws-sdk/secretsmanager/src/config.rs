// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

pub struct Config {
    pub(crate) token_provider: Box<dyn crate::idempotency_token::ProvideIdempotencyToken>,
    pub endpoint_resolver: ::std::sync::Arc<dyn ::aws_endpoint::ResolveAwsEndpoint>,
    pub(crate) region: Option<::aws_types::region::Region>,
    pub(crate) credentials_provider: std::sync::Arc<dyn ::aws_auth::ProvideCredentials>,
}
impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// The signature version 4 service signing name to use in the credential scope when signing requests.
    ///
    /// The signing service may be overidden by the `Endpoint`, or by specifying a custom [`SigningService`](aws_types::SigningService) during
    /// operation construction
    pub fn signing_service(&self) -> &'static str {
        "secretsmanager"
    }
}
#[derive(Default)]
pub struct ConfigBuilder {
    token_provider: Option<Box<dyn crate::idempotency_token::ProvideIdempotencyToken>>,
    endpoint_resolver: Option<::std::sync::Arc<dyn ::aws_endpoint::ResolveAwsEndpoint>>,
    region: Option<::aws_types::region::Region>,
    credentials_provider: Option<std::sync::Arc<dyn ::aws_auth::ProvideCredentials>>,
}
impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token_provider(
        mut self,
        token_provider: impl crate::idempotency_token::ProvideIdempotencyToken + 'static,
    ) -> Self {
        self.token_provider = Some(Box::new(token_provider));
        self
    }

    pub fn endpoint_resolver(
        mut self,
        endpoint_resolver: impl ::aws_endpoint::ResolveAwsEndpoint + 'static,
    ) -> Self {
        self.endpoint_resolver = Some(::std::sync::Arc::new(endpoint_resolver));
        self
    }

    pub fn region(mut self, region_provider: impl ::aws_types::region::ProvideRegion) -> Self {
        self.region = region_provider.region();
        self
    }

    /// Set the credentials provider for this service

    pub fn credentials_provider(
        mut self,
        credentials_provider: impl ::aws_auth::ProvideCredentials + 'static,
    ) -> Self {
        self.credentials_provider = Some(std::sync::Arc::new(credentials_provider));
        self
    }

    pub fn build(self) -> Config {
        Config {
            token_provider: self
                .token_provider
                .unwrap_or_else(|| Box::new(crate::idempotency_token::default_provider())),
            endpoint_resolver: self.endpoint_resolver.unwrap_or_else(|| {
                ::std::sync::Arc::new(::aws_endpoint::DefaultAwsEndpointResolver::for_service(
                    "secretsmanager",
                ))
            }),
            region: {
                use ::aws_types::region::ProvideRegion;
                self.region
                    .or_else(|| ::aws_types::region::default_provider().region())
            },
            credentials_provider: self
                .credentials_provider
                .unwrap_or_else(|| std::sync::Arc::new(::aws_auth::default_provider())),
        }
    }
}
