// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetDeviceRegistrationInputBody<'a> {
    /// <p>The unique name of the device you want to get the registration status from.</p>
    pub device_name: &'a std::option::Option<std::string::String>,
    /// <p>The name of the fleet that the device belongs to.</p>
    pub device_fleet_name: &'a std::option::Option<std::string::String>,
}
impl<'a> std::fmt::Debug for GetDeviceRegistrationInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetDeviceRegistrationInputBody");
        formatter.field("device_name", &self.device_name);
        formatter.field("device_fleet_name", &self.device_fleet_name);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendHeartbeatInputBody<'a> {
    /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
    pub agent_metrics: &'a std::option::Option<std::vec::Vec<crate::model::EdgeMetric>>,
    /// <p>Returns a list of models deployed on the the device.</p>
    pub models: &'a std::option::Option<std::vec::Vec<crate::model::Model>>,
    /// <p>Returns the version of the agent.</p>
    pub agent_version: &'a std::option::Option<std::string::String>,
    /// <p>The unique name of the device.</p>
    pub device_name: &'a std::option::Option<std::string::String>,
    /// <p>The name of the fleet that the device belongs to.</p>
    pub device_fleet_name: &'a std::option::Option<std::string::String>,
}
impl<'a> std::fmt::Debug for SendHeartbeatInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendHeartbeatInputBody");
        formatter.field("agent_metrics", &self.agent_metrics);
        formatter.field("models", &self.models);
        formatter.field("agent_version", &self.agent_version);
        formatter.field("device_name", &self.device_name);
        formatter.field("device_fleet_name", &self.device_fleet_name);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct GetDeviceRegistrationOutputBody {
    /// <p>Describes if the device is currently registered with SageMaker Edge Manager.</p>
    #[serde(rename = "DeviceRegistration")]
    #[serde(default)]
    pub device_registration: std::option::Option<std::string::String>,
    /// <p>The amount of time, in seconds, that the registration status is stored on the device’s cache before it is refreshed.</p>
    #[serde(rename = "CacheTTL")]
    #[serde(default)]
    pub cache_ttl: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for GetDeviceRegistrationOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetDeviceRegistrationOutputBody");
        formatter.field("device_registration", &self.device_registration);
        formatter.field("cache_ttl", &self.cache_ttl);
        formatter.finish()
    }
}
