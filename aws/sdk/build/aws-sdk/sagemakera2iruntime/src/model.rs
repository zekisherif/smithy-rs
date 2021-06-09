// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Attributes of the data specified by the customer. Use these to describe the data to be labeled.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct HumanLoopDataAttributes {
    /// <p>Declares that your content is free of personally identifiable information or adult content.</p>
    /// <p>Amazon SageMaker can restrict the Amazon Mechanical Turk workers who can view your task based on this information.</p>
    #[serde(rename = "ContentClassifiers")]
    #[serde(default)]
    pub content_classifiers: std::option::Option<std::vec::Vec<crate::model::ContentClassifier>>,
}
impl std::fmt::Debug for HumanLoopDataAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HumanLoopDataAttributes");
        formatter.field("content_classifiers", &self.content_classifiers);
        formatter.finish()
    }
}
/// See [`HumanLoopDataAttributes`](crate::model::HumanLoopDataAttributes)
pub mod human_loop_data_attributes {
    /// A builder for [`HumanLoopDataAttributes`](crate::model::HumanLoopDataAttributes)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) content_classifiers:
            std::option::Option<std::vec::Vec<crate::model::ContentClassifier>>,
    }
    impl Builder {
        pub fn content_classifiers(
            mut self,
            inp: impl Into<crate::model::ContentClassifier>,
        ) -> Self {
            let mut v = self.content_classifiers.unwrap_or_default();
            v.push(inp.into());
            self.content_classifiers = Some(v);
            self
        }
        pub fn set_content_classifiers(
            mut self,
            inp: std::option::Option<std::vec::Vec<crate::model::ContentClassifier>>,
        ) -> Self {
            self.content_classifiers = inp;
            self
        }
        /// Consumes the builder and constructs a [`HumanLoopDataAttributes`](crate::model::HumanLoopDataAttributes)
        pub fn build(self) -> crate::model::HumanLoopDataAttributes {
            crate::model::HumanLoopDataAttributes {
                content_classifiers: self.content_classifiers,
            }
        }
    }
}
impl HumanLoopDataAttributes {
    /// Creates a new builder-style object to manufacture [`HumanLoopDataAttributes`](crate::model::HumanLoopDataAttributes)
    pub fn builder() -> crate::model::human_loop_data_attributes::Builder {
        crate::model::human_loop_data_attributes::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ContentClassifier {
    FreeOfAdultContent,
    FreeOfPersonallyIdentifiableInformation,
    Unknown(String),
}
impl std::convert::From<&str> for ContentClassifier {
    fn from(s: &str) -> Self {
        match s {
            "FreeOfAdultContent" => ContentClassifier::FreeOfAdultContent,
            "FreeOfPersonallyIdentifiableInformation" => {
                ContentClassifier::FreeOfPersonallyIdentifiableInformation
            }
            other => ContentClassifier::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ContentClassifier {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ContentClassifier::from(s))
    }
}
impl ContentClassifier {
    pub fn as_str(&self) -> &str {
        match self {
            ContentClassifier::FreeOfAdultContent => "FreeOfAdultContent",
            ContentClassifier::FreeOfPersonallyIdentifiableInformation => {
                "FreeOfPersonallyIdentifiableInformation"
            }
            ContentClassifier::Unknown(s) => s.as_ref(),
        }
    }
}
impl AsRef<str> for ContentClassifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl<'de> serde::Deserialize<'de> for ContentClassifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = <&str>::deserialize(deserializer)?;
        Ok(Self::from(data))
    }
}

/// <p>An object containing the human loop input in JSON format.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct HumanLoopInput {
    /// <p>Serialized input from the human loop. The input must be a string representation of a file in JSON format.</p>
    #[serde(rename = "InputContent")]
    #[serde(default)]
    pub input_content: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for HumanLoopInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HumanLoopInput");
        formatter.field("input_content", &self.input_content);
        formatter.finish()
    }
}
/// See [`HumanLoopInput`](crate::model::HumanLoopInput)
pub mod human_loop_input {
    /// A builder for [`HumanLoopInput`](crate::model::HumanLoopInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) input_content: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Serialized input from the human loop. The input must be a string representation of a file in JSON format.</p>
        pub fn input_content(mut self, inp: impl Into<std::string::String>) -> Self {
            self.input_content = Some(inp.into());
            self
        }
        pub fn set_input_content(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.input_content = inp;
            self
        }
        /// Consumes the builder and constructs a [`HumanLoopInput`](crate::model::HumanLoopInput)
        pub fn build(self) -> crate::model::HumanLoopInput {
            crate::model::HumanLoopInput {
                input_content: self.input_content,
            }
        }
    }
}
impl HumanLoopInput {
    /// Creates a new builder-style object to manufacture [`HumanLoopInput`](crate::model::HumanLoopInput)
    pub fn builder() -> crate::model::human_loop_input::Builder {
        crate::model::human_loop_input::Builder::default()
    }
}

/// <p>Summary information about the human loop.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct HumanLoopSummary {
    /// <p>The name of the human loop.</p>
    #[serde(rename = "HumanLoopName")]
    #[serde(default)]
    pub human_loop_name: std::option::Option<std::string::String>,
    /// <p>The status of the human loop. </p>
    #[serde(rename = "HumanLoopStatus")]
    #[serde(default)]
    pub human_loop_status: std::option::Option<crate::model::HumanLoopStatus>,
    /// <p>When Amazon Augmented AI created the human loop.</p>
    #[serde(rename = "CreationTime")]
    #[serde(
        deserialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_deser"
    )]
    #[serde(default)]
    pub creation_time: std::option::Option<smithy_types::Instant>,
    /// <p>The reason why the human loop failed. A failure reason is returned when the status of the
    /// human loop is <code>Failed</code>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    pub failure_reason: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition used to configure the human
    /// loop.</p>
    #[serde(rename = "FlowDefinitionArn")]
    #[serde(default)]
    pub flow_definition_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for HumanLoopSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HumanLoopSummary");
        formatter.field("human_loop_name", &self.human_loop_name);
        formatter.field("human_loop_status", &self.human_loop_status);
        formatter.field("creation_time", &self.creation_time);
        formatter.field("failure_reason", &self.failure_reason);
        formatter.field("flow_definition_arn", &self.flow_definition_arn);
        formatter.finish()
    }
}
/// See [`HumanLoopSummary`](crate::model::HumanLoopSummary)
pub mod human_loop_summary {
    /// A builder for [`HumanLoopSummary`](crate::model::HumanLoopSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) human_loop_name: std::option::Option<std::string::String>,
        pub(crate) human_loop_status: std::option::Option<crate::model::HumanLoopStatus>,
        pub(crate) creation_time: std::option::Option<smithy_types::Instant>,
        pub(crate) failure_reason: std::option::Option<std::string::String>,
        pub(crate) flow_definition_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the human loop.</p>
        pub fn human_loop_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.human_loop_name = Some(inp.into());
            self
        }
        pub fn set_human_loop_name(
            mut self,
            inp: std::option::Option<std::string::String>,
        ) -> Self {
            self.human_loop_name = inp;
            self
        }
        /// <p>The status of the human loop. </p>
        pub fn human_loop_status(mut self, inp: crate::model::HumanLoopStatus) -> Self {
            self.human_loop_status = Some(inp);
            self
        }
        pub fn set_human_loop_status(
            mut self,
            inp: std::option::Option<crate::model::HumanLoopStatus>,
        ) -> Self {
            self.human_loop_status = inp;
            self
        }
        /// <p>When Amazon Augmented AI created the human loop.</p>
        pub fn creation_time(mut self, inp: smithy_types::Instant) -> Self {
            self.creation_time = Some(inp);
            self
        }
        pub fn set_creation_time(
            mut self,
            inp: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.creation_time = inp;
            self
        }
        /// <p>The reason why the human loop failed. A failure reason is returned when the status of the
        /// human loop is <code>Failed</code>.</p>
        pub fn failure_reason(mut self, inp: impl Into<std::string::String>) -> Self {
            self.failure_reason = Some(inp.into());
            self
        }
        pub fn set_failure_reason(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.failure_reason = inp;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the flow definition used to configure the human
        /// loop.</p>
        pub fn flow_definition_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.flow_definition_arn = Some(inp.into());
            self
        }
        pub fn set_flow_definition_arn(
            mut self,
            inp: std::option::Option<std::string::String>,
        ) -> Self {
            self.flow_definition_arn = inp;
            self
        }
        /// Consumes the builder and constructs a [`HumanLoopSummary`](crate::model::HumanLoopSummary)
        pub fn build(self) -> crate::model::HumanLoopSummary {
            crate::model::HumanLoopSummary {
                human_loop_name: self.human_loop_name,
                human_loop_status: self.human_loop_status,
                creation_time: self.creation_time,
                failure_reason: self.failure_reason,
                flow_definition_arn: self.flow_definition_arn,
            }
        }
    }
}
impl HumanLoopSummary {
    /// Creates a new builder-style object to manufacture [`HumanLoopSummary`](crate::model::HumanLoopSummary)
    pub fn builder() -> crate::model::human_loop_summary::Builder {
        crate::model::human_loop_summary::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum HumanLoopStatus {
    Completed,
    Failed,
    InProgress,
    Stopped,
    Stopping,
    Unknown(String),
}
impl std::convert::From<&str> for HumanLoopStatus {
    fn from(s: &str) -> Self {
        match s {
            "Completed" => HumanLoopStatus::Completed,
            "Failed" => HumanLoopStatus::Failed,
            "InProgress" => HumanLoopStatus::InProgress,
            "Stopped" => HumanLoopStatus::Stopped,
            "Stopping" => HumanLoopStatus::Stopping,
            other => HumanLoopStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for HumanLoopStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HumanLoopStatus::from(s))
    }
}
impl HumanLoopStatus {
    pub fn as_str(&self) -> &str {
        match self {
            HumanLoopStatus::Completed => "Completed",
            HumanLoopStatus::Failed => "Failed",
            HumanLoopStatus::InProgress => "InProgress",
            HumanLoopStatus::Stopped => "Stopped",
            HumanLoopStatus::Stopping => "Stopping",
            HumanLoopStatus::Unknown(s) => s.as_ref(),
        }
    }
}
impl AsRef<str> for HumanLoopStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl<'de> serde::Deserialize<'de> for HumanLoopStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = <&str>::deserialize(deserializer)?;
        Ok(Self::from(data))
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum SortOrder {
    Ascending,
    Descending,
    Unknown(String),
}
impl std::convert::From<&str> for SortOrder {
    fn from(s: &str) -> Self {
        match s {
            "Ascending" => SortOrder::Ascending,
            "Descending" => SortOrder::Descending,
            other => SortOrder::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for SortOrder {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SortOrder::from(s))
    }
}
impl SortOrder {
    pub fn as_str(&self) -> &str {
        match self {
            SortOrder::Ascending => "Ascending",
            SortOrder::Descending => "Descending",
            SortOrder::Unknown(s) => s.as_ref(),
        }
    }
}
impl AsRef<str> for SortOrder {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl<'de> serde::Deserialize<'de> for SortOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = <&str>::deserialize(deserializer)?;
        Ok(Self::from(data))
    }
}

/// <p>Information about where the human output will be stored.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct HumanLoopOutput {
    /// <p>The location of the Amazon S3 object where Amazon Augmented AI stores your human loop output.</p>
    #[serde(rename = "OutputS3Uri")]
    #[serde(default)]
    pub output_s3_uri: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for HumanLoopOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HumanLoopOutput");
        formatter.field("output_s3_uri", &self.output_s3_uri);
        formatter.finish()
    }
}
/// See [`HumanLoopOutput`](crate::model::HumanLoopOutput)
pub mod human_loop_output {
    /// A builder for [`HumanLoopOutput`](crate::model::HumanLoopOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) output_s3_uri: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The location of the Amazon S3 object where Amazon Augmented AI stores your human loop output.</p>
        pub fn output_s3_uri(mut self, inp: impl Into<std::string::String>) -> Self {
            self.output_s3_uri = Some(inp.into());
            self
        }
        pub fn set_output_s3_uri(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.output_s3_uri = inp;
            self
        }
        /// Consumes the builder and constructs a [`HumanLoopOutput`](crate::model::HumanLoopOutput)
        pub fn build(self) -> crate::model::HumanLoopOutput {
            crate::model::HumanLoopOutput {
                output_s3_uri: self.output_s3_uri,
            }
        }
    }
}
impl HumanLoopOutput {
    /// Creates a new builder-style object to manufacture [`HumanLoopOutput`](crate::model::HumanLoopOutput)
    pub fn builder() -> crate::model::human_loop_output::Builder {
        crate::model::human_loop_output::Builder::default()
    }
}
