// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(std::fmt::Debug)]
pub(crate) struct Handle<C = aws_hyper::DynConnector> {
    client: aws_hyper::Client<C>,
    conf: crate::Config,
}

#[derive(Clone, std::fmt::Debug)]
pub struct Client<C = aws_hyper::DynConnector> {
    handle: std::sync::Arc<Handle<C>>,
}
impl<C> Client<C> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_env() -> Self {
        Self::from_conf(crate::Config::builder().build())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl<C> Client<C>
where
    C: aws_hyper::SmithyConnector,
{
    pub fn invoke_endpoint(&self) -> fluent_builders::InvokeEndpoint<C> {
        fluent_builders::InvokeEndpoint::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct InvokeEndpoint<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::invoke_endpoint_input::Builder,
    }
    impl<C> InvokeEndpoint<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> Result<
            crate::output::InvokeEndpointOutput,
            smithy_http::result::SdkError<crate::error::InvokeEndpointError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the endpoint that you specified when you created the endpoint using the
        /// <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API. </p>
        pub fn endpoint_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.endpoint_name(inp);
            self
        }
        pub fn set_endpoint_name(mut self, inp: std::string::String) -> Self {
            self.inner = self.inner.set_endpoint_name(inp);
            self
        }
        /// <p>Provides input data, in the format specified in the <code>ContentType</code>
        /// request header. Amazon SageMaker passes all of the data in the body to the model. </p>
        /// <p>For information about the format of the request body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data
        /// Formats-Inference</a>.</p>
        pub fn body(mut self, inp: smithy_types::Blob) -> Self {
            self.inner = self.inner.body(inp);
            self
        }
        pub fn set_body(mut self, inp: std::option::Option<smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_body(inp);
            self
        }
        /// <p>The MIME type of the input data in the request body.</p>
        pub fn content_type(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.content_type(inp);
            self
        }
        pub fn set_content_type(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_content_type(inp);
            self
        }
        /// <p>The desired MIME type of the inference in the response.</p>
        pub fn accept(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.accept(inp);
            self
        }
        pub fn set_accept(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_accept(inp);
            self
        }
        /// <p>Provides additional information about a request for an inference submitted to a model
        /// hosted at an Amazon SageMaker endpoint. The information is an opaque value that is
        /// forwarded verbatim. You could use this value, for example, to provide an ID that you can
        /// use to track a request or to provide other metadata that a service endpoint was
        /// programmed to process. The value must consist of no more than 1024 visible US-ASCII
        /// characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value
        /// Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p>
        /// <p>The code in your model is responsible for setting or updating any custom attributes in
        /// the response. If your code does not set this value in the response, an empty value is
        /// returned. For example, if a custom attribute represents the trace ID, your model can
        /// prepend the custom attribute with <code>Trace ID:</code> in your post-processing
        /// function.</p>
        /// <p>This feature is currently supported in the AWS SDKs but not in the Amazon SageMaker Python
        /// SDK.</p>
        pub fn custom_attributes(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.custom_attributes(inp);
            self
        }
        pub fn set_custom_attributes(
            mut self,
            inp: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_custom_attributes(inp);
            self
        }
        /// <p>The model to request for inference when invoking a multi-model endpoint.</p>
        pub fn target_model(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_model(inp);
            self
        }
        pub fn set_target_model(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_target_model(inp);
            self
        }
        /// <p>Specify the production variant to send the inference request to when invoking an
        /// endpoint that is running two or more variants. Note that this parameter overrides the
        /// default behavior for the endpoint, which is to distribute the invocation traffic based
        /// on the variant weights.</p>
        /// <p>For information about how to use variant targeting to perform a/b testing, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-ab-testing.html">Test models in
        /// production</a>
        /// </p>
        pub fn target_variant(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_variant(inp);
            self
        }
        pub fn set_target_variant(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_target_variant(inp);
            self
        }
        /// <p>If the endpoint hosts multiple containers and is configured to use direct invocation,
        /// this parameter specifies the host name of the container to invoke.</p>
        pub fn target_container_hostname(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_container_hostname(inp);
            self
        }
        pub fn set_target_container_hostname(
            mut self,
            inp: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_target_container_hostname(inp);
            self
        }
        /// <p>If you provide a value, it is added to the captured data when you enable data capture
        /// on the endpoint. For information about data capture, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture.html">Capture
        /// Data</a>.</p>
        pub fn inference_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.inference_id(inp);
            self
        }
        pub fn set_inference_id(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_inference_id(inp);
            self
        }
    }
}
