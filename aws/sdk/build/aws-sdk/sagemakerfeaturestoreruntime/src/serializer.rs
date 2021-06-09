// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutRecordInputBody<'a> {
    /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want
    /// to update few of the feature values, do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Use <code>GetRecord</code> to retrieve the latest record.</p>
    /// </li>
    /// <li>
    /// <p>Update the record returned from <code>GetRecord</code>. </p>
    /// </li>
    /// <li>
    /// <p>Use <code>PutRecord</code> to update feature values.</p>
    /// </li>
    /// </ul>
    pub record: &'a std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
}
impl<'a> std::fmt::Debug for PutRecordInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutRecordInputBody");
        formatter.field("record", &self.record);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRecordOutputBody {
    /// <p>The record you requested. A list of <code>FeatureValues</code>.</p>
    #[serde(rename = "Record")]
    #[serde(default)]
    pub record: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
}
impl std::fmt::Debug for GetRecordOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRecordOutputBody");
        formatter.field("record", &self.record);
        formatter.finish()
    }
}
