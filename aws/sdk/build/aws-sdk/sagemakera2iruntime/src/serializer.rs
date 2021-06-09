// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartHumanLoopInputBody<'a> {
    /// <p>The name of the human loop.</p>
    pub human_loop_name: &'a std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition associated with this human
    /// loop.</p>
    pub flow_definition_arn: &'a std::option::Option<std::string::String>,
    /// <p>An object that contains information about the human loop.</p>
    pub human_loop_input: &'a std::option::Option<crate::model::HumanLoopInput>,
    /// <p>Attributes of the specified data. Use <code>DataAttributes</code> to specify if your data
    /// is free of personally identifiable information and/or free of adult content.</p>
    pub data_attributes: &'a std::option::Option<crate::model::HumanLoopDataAttributes>,
}
impl<'a> std::fmt::Debug for StartHumanLoopInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartHumanLoopInputBody");
        formatter.field("human_loop_name", &self.human_loop_name);
        formatter.field("flow_definition_arn", &self.flow_definition_arn);
        formatter.field("human_loop_input", &self.human_loop_input);
        formatter.field("data_attributes", &self.data_attributes);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StopHumanLoopInputBody<'a> {
    /// <p>The name of the human loop that you want to stop.</p>
    pub human_loop_name: &'a std::option::Option<std::string::String>,
}
impl<'a> std::fmt::Debug for StopHumanLoopInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StopHumanLoopInputBody");
        formatter.field("human_loop_name", &self.human_loop_name);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeHumanLoopOutputBody {
    /// <p>The creation time when Amazon Augmented AI created the human loop.</p>
    #[serde(rename = "CreationTime")]
    #[serde(
        deserialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_deser"
    )]
    #[serde(default)]
    pub creation_time: std::option::Option<smithy_types::Instant>,
    /// <p>The reason why a human loop failed. The failure reason is returned when the status of the
    /// human loop is <code>Failed</code>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    pub failure_reason: std::option::Option<std::string::String>,
    /// <p>A failure code that identifies the type of failure.</p>
    /// <p>Possible values: <code>ValidationError</code>, <code>Expired</code>,
    /// <code>InternalError</code>
    /// </p>
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    pub failure_code: std::option::Option<std::string::String>,
    /// <p>The status of the human loop. </p>
    #[serde(rename = "HumanLoopStatus")]
    #[serde(default)]
    pub human_loop_status: std::option::Option<crate::model::HumanLoopStatus>,
    /// <p>The name of the human loop. The name must be lowercase, unique within the Region in your
    /// account, and can have up to 63 characters. Valid characters: a-z, 0-9, and - (hyphen).</p>
    #[serde(rename = "HumanLoopName")]
    #[serde(default)]
    pub human_loop_name: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    #[serde(rename = "HumanLoopArn")]
    #[serde(default)]
    pub human_loop_arn: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    #[serde(rename = "FlowDefinitionArn")]
    #[serde(default)]
    pub flow_definition_arn: std::option::Option<std::string::String>,
    /// <p>An object that contains information about the output of the human loop.</p>
    #[serde(rename = "HumanLoopOutput")]
    #[serde(default)]
    pub human_loop_output: std::option::Option<crate::model::HumanLoopOutput>,
}
impl std::fmt::Debug for DescribeHumanLoopOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeHumanLoopOutputBody");
        formatter.field("creation_time", &self.creation_time);
        formatter.field("failure_reason", &self.failure_reason);
        formatter.field("failure_code", &self.failure_code);
        formatter.field("human_loop_status", &self.human_loop_status);
        formatter.field("human_loop_name", &self.human_loop_name);
        formatter.field("human_loop_arn", &self.human_loop_arn);
        formatter.field("flow_definition_arn", &self.flow_definition_arn);
        formatter.field("human_loop_output", &self.human_loop_output);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct ListHumanLoopsOutputBody {
    /// <p>An array of objects that contain information about the human loops.</p>
    #[serde(rename = "HumanLoopSummaries")]
    #[serde(default)]
    pub human_loop_summaries: std::option::Option<std::vec::Vec<crate::model::HumanLoopSummary>>,
    /// <p>A token to display the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(default)]
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListHumanLoopsOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListHumanLoopsOutputBody");
        formatter.field("human_loop_summaries", &self.human_loop_summaries);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct StartHumanLoopOutputBody {
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    #[serde(rename = "HumanLoopArn")]
    #[serde(default)]
    pub human_loop_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for StartHumanLoopOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartHumanLoopOutputBody");
        formatter.field("human_loop_arn", &self.human_loop_arn);
        formatter.finish()
    }
}
