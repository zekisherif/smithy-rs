// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Displays errors that occurred during validation of the resource policy.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct ValidationErrorsEntry {
    /// <p>Displays error messages if validation encounters problems during validation of the resource policy.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub error_message: std::option::Option<std::string::String>,
    /// <p>Checks the name of the policy.</p>
    #[serde(rename = "CheckName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub check_name: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ValidationErrorsEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ValidationErrorsEntry");
        formatter.field("error_message", &self.error_message);
        formatter.field("check_name", &self.check_name);
        formatter.finish()
    }
}
/// See [`ValidationErrorsEntry`](crate::model::ValidationErrorsEntry)
pub mod validation_errors_entry {

    /// A builder for [`ValidationErrorsEntry`](crate::model::ValidationErrorsEntry)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        error_message: std::option::Option<std::string::String>,
        check_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Displays error messages if validation encounters problems during validation of the resource policy.</p>
        pub fn error_message(mut self, inp: impl Into<std::string::String>) -> Self {
            self.error_message = Some(inp.into());
            self
        }
        pub fn set_error_message(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.error_message = inp;
            self
        }
        /// <p>Checks the name of the policy.</p>
        pub fn check_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.check_name = Some(inp.into());
            self
        }
        pub fn set_check_name(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.check_name = inp;
            self
        }
        /// Consumes the builder and constructs a [`ValidationErrorsEntry`](crate::model::ValidationErrorsEntry)
        pub fn build(self) -> crate::model::ValidationErrorsEntry {
            crate::model::ValidationErrorsEntry {
                error_message: self.error_message,
                check_name: self.check_name,
            }
        }
    }
}
impl ValidationErrorsEntry {
    /// Creates a new builder-style object to manufacture [`ValidationErrorsEntry`](crate::model::ValidationErrorsEntry)
    pub fn builder() -> crate::model::validation_errors_entry::Builder {
        crate::model::validation_errors_entry::Builder::default()
    }
}

/// <p>A structure that contains information about a tag.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>The string value associated with the key of the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: std::option::Option<std::string::String>,
    /// <p>The key identifier, or name, of the tag.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub key: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("value", &self.value);
        formatter.field("key", &self.key);
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {

    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        value: std::option::Option<std::string::String>,
        key: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The string value associated with the key of the tag.</p>
        pub fn value(mut self, inp: impl Into<std::string::String>) -> Self {
            self.value = Some(inp.into());
            self
        }
        pub fn set_value(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.value = inp;
            self
        }
        /// <p>The key identifier, or name, of the tag.</p>
        pub fn key(mut self, inp: impl Into<std::string::String>) -> Self {
            self.key = Some(inp.into());
            self
        }
        pub fn set_key(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.key = inp;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                value: self.value,
                key: self.key,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

/// <p>A structure that defines the rotation configuration for the secret.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct RotationRulesType {
    /// <p>Specifies the number of days between automatic scheduled rotations of the secret.</p>
    /// <p>Secrets Manager schedules the next rotation when the previous
    /// one is complete. Secrets Manager schedules the date by adding the rotation interval (number of days) to the
    /// actual date of the last rotation. The service chooses the hour within that 24-hour date window
    /// randomly. The minute is also chosen somewhat randomly, but weighted towards the top of the hour
    /// and influenced by a variety of factors that help distribute load.</p>
    #[serde(rename = "AutomaticallyAfterDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub automatically_after_days: std::option::Option<i64>,
}
impl std::fmt::Debug for RotationRulesType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RotationRulesType");
        formatter.field("automatically_after_days", &self.automatically_after_days);
        formatter.finish()
    }
}
/// See [`RotationRulesType`](crate::model::RotationRulesType)
pub mod rotation_rules_type {

    /// A builder for [`RotationRulesType`](crate::model::RotationRulesType)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        automatically_after_days: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>Specifies the number of days between automatic scheduled rotations of the secret.</p>
        /// <p>Secrets Manager schedules the next rotation when the previous
        /// one is complete. Secrets Manager schedules the date by adding the rotation interval (number of days) to the
        /// actual date of the last rotation. The service chooses the hour within that 24-hour date window
        /// randomly. The minute is also chosen somewhat randomly, but weighted towards the top of the hour
        /// and influenced by a variety of factors that help distribute load.</p>
        pub fn automatically_after_days(mut self, inp: i64) -> Self {
            self.automatically_after_days = Some(inp);
            self
        }
        pub fn set_automatically_after_days(mut self, inp: std::option::Option<i64>) -> Self {
            self.automatically_after_days = inp;
            self
        }
        /// Consumes the builder and constructs a [`RotationRulesType`](crate::model::RotationRulesType)
        pub fn build(self) -> crate::model::RotationRulesType {
            crate::model::RotationRulesType {
                automatically_after_days: self.automatically_after_days,
            }
        }
    }
}
impl RotationRulesType {
    /// Creates a new builder-style object to manufacture [`RotationRulesType`](crate::model::RotationRulesType)
    pub fn builder() -> crate::model::rotation_rules_type::Builder {
        crate::model::rotation_rules_type::Builder::default()
    }
}

/// <p>A structure that contains information about one version of a secret.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct SecretVersionsListEntry {
    /// <p>An array of staging labels that are currently associated with this version of the
    /// secret.</p>
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub version_stages: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The date and time this version of the secret was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_ser"
    )]
    #[serde(
        deserialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_deser"
    )]
    #[serde(default)]
    pub created_date: std::option::Option<smithy_types::Instant>,
    /// <p>The unique version identifier of this version of the secret.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub version_id: std::option::Option<std::string::String>,
    /// <p>The date that this version of the secret was last accessed. Note that the resolution of
    /// this field is at the date level and does not include the time.</p>
    #[serde(rename = "LastAccessedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_ser"
    )]
    #[serde(
        deserialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_deser"
    )]
    #[serde(default)]
    pub last_accessed_date: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for SecretVersionsListEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SecretVersionsListEntry");
        formatter.field("version_stages", &self.version_stages);
        formatter.field("created_date", &self.created_date);
        formatter.field("version_id", &self.version_id);
        formatter.field("last_accessed_date", &self.last_accessed_date);
        formatter.finish()
    }
}
/// See [`SecretVersionsListEntry`](crate::model::SecretVersionsListEntry)
pub mod secret_versions_list_entry {

    /// A builder for [`SecretVersionsListEntry`](crate::model::SecretVersionsListEntry)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        version_stages: std::option::Option<std::vec::Vec<std::string::String>>,
        created_date: std::option::Option<smithy_types::Instant>,
        version_id: std::option::Option<std::string::String>,
        last_accessed_date: std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        pub fn version_stages(mut self, inp: impl Into<std::string::String>) -> Self {
            let mut v = self.version_stages.unwrap_or_default();
            v.push(inp.into());
            self.version_stages = Some(v);
            self
        }
        pub fn set_version_stages(
            mut self,
            inp: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.version_stages = inp;
            self
        }
        /// <p>The date and time this version of the secret was created.</p>
        pub fn created_date(mut self, inp: smithy_types::Instant) -> Self {
            self.created_date = Some(inp);
            self
        }
        pub fn set_created_date(mut self, inp: std::option::Option<smithy_types::Instant>) -> Self {
            self.created_date = inp;
            self
        }
        /// <p>The unique version identifier of this version of the secret.</p>
        pub fn version_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.version_id = Some(inp.into());
            self
        }
        pub fn set_version_id(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.version_id = inp;
            self
        }
        /// <p>The date that this version of the secret was last accessed. Note that the resolution of
        /// this field is at the date level and does not include the time.</p>
        pub fn last_accessed_date(mut self, inp: smithy_types::Instant) -> Self {
            self.last_accessed_date = Some(inp);
            self
        }
        pub fn set_last_accessed_date(
            mut self,
            inp: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.last_accessed_date = inp;
            self
        }
        /// Consumes the builder and constructs a [`SecretVersionsListEntry`](crate::model::SecretVersionsListEntry)
        pub fn build(self) -> crate::model::SecretVersionsListEntry {
            crate::model::SecretVersionsListEntry {
                version_stages: self.version_stages,
                created_date: self.created_date,
                version_id: self.version_id,
                last_accessed_date: self.last_accessed_date,
            }
        }
    }
}
impl SecretVersionsListEntry {
    /// Creates a new builder-style object to manufacture [`SecretVersionsListEntry`](crate::model::SecretVersionsListEntry)
    pub fn builder() -> crate::model::secret_versions_list_entry::Builder {
        crate::model::secret_versions_list_entry::Builder::default()
    }
}

/// <p>A structure that contains the details about a secret. It does not include the encrypted
/// <code>SecretString</code> and <code>SecretBinary</code> values. To get those values, use the
/// <a>GetSecretValue</a> operation.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct SecretListEntry {
    /// <p>The last date and time that the rotation process for this secret was invoked.</p>
    #[serde(rename = "LastRotatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_ser"
    )]
    #[serde(
        deserialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_deser"
    )]
    #[serde(default)]
    pub last_rotated_date: std::option::Option<smithy_types::Instant>,
    /// <p>The user-provided description of the secret.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The ARN or alias of the AWS KMS customer master key (CMK) used to encrypt the
    /// <code>SecretString</code> and <code>SecretBinary</code> fields in each version of the
    /// secret. If you don't provide a key, then Secrets Manager defaults to encrypting the secret fields with
    /// the default KMS CMK, the key named <code>awssecretsmanager</code>, for this account.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub kms_key_id: std::option::Option<std::string::String>,
    /// <p>A list of all of the currently assigned <code>SecretVersionStage</code> staging labels and
    /// the <code>SecretVersionId</code> attached to each one. Staging labels are used to keep
    /// track of the different versions during the rotation process.</p>
    /// <note>
    /// <p>A version that does not have any <code>SecretVersionStage</code> is considered
    /// deprecated and subject to deletion. Such versions are not included in this list.</p>
    /// </note>
    #[serde(rename = "SecretVersionsToStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub secret_versions_to_stages: std::option::Option<
        std::collections::HashMap<std::string::String, std::vec::Vec<std::string::String>>,
    >,
    /// <p>The date and time when a secret was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_ser"
    )]
    #[serde(
        deserialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_deser"
    )]
    #[serde(default)]
    pub created_date: std::option::Option<smithy_types::Instant>,
    /// <p>The date and time the deletion of the secret occurred. Not present on active secrets. The
    /// secret can be recovered until the number of days in the recovery window has passed, as
    /// specified in the <code>RecoveryWindowInDays</code> parameter of the <a>DeleteSecret</a> operation.</p>
    #[serde(rename = "DeletedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_ser"
    )]
    #[serde(
        deserialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_deser"
    )]
    #[serde(default)]
    pub deleted_date: std::option::Option<smithy_types::Instant>,
    /// <p>The Amazon Resource Name (ARN) of the secret.</p>
    /// <p>For more information about ARNs in Secrets Manager, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#iam-resources">Policy Resources</a> in the
    /// <i>AWS Secrets Manager User Guide</i>.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>Returns the name of the service that created the secret.</p>
    #[serde(rename = "OwningService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub owning_service: std::option::Option<std::string::String>,
    /// <p>The ARN of an AWS Lambda function invoked by Secrets Manager to rotate and expire the
    /// secret either automatically per the schedule or manually by a call to <a>RotateSecret</a>.</p>
    #[serde(rename = "RotationLambdaARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rotation_lambda_arn: std::option::Option<std::string::String>,
    /// <p>The list of user-defined tags associated with the secret. To add tags to a
    /// secret, use <a>TagResource</a>. To remove tags, use <a>UntagResource</a>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    /// <p>Indicates whether automatic, scheduled rotation is enabled for this secret.</p>
    #[serde(rename = "RotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rotation_enabled: std::option::Option<bool>,
    /// <p>The last date that this secret was accessed. This value is truncated to midnight of the
    /// date and therefore shows only the date, not the time.</p>
    #[serde(rename = "LastAccessedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_ser"
    )]
    #[serde(
        deserialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_deser"
    )]
    #[serde(default)]
    pub last_accessed_date: std::option::Option<smithy_types::Instant>,
    /// <p>The last date and time that this secret was modified in any way.</p>
    #[serde(rename = "LastChangedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_ser"
    )]
    #[serde(
        deserialize_with = "crate::serde_util::stdoptionoptionsmithytypesinstant_epoch_seconds_deser"
    )]
    #[serde(default)]
    pub last_changed_date: std::option::Option<smithy_types::Instant>,
    /// <p>A structure that defines the rotation configuration for the secret.</p>
    #[serde(rename = "RotationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rotation_rules: std::option::Option<crate::model::RotationRulesType>,
    /// <p>The friendly name of the secret. You can use forward slashes in the name to represent a
    /// path hierarchy. For example, <code>/prod/databases/dbserver1</code> could represent the secret
    /// for a server named <code>dbserver1</code> in the folder <code>databases</code> in the folder
    /// <code>prod</code>. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for SecretListEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SecretListEntry");
        formatter.field("last_rotated_date", &self.last_rotated_date);
        formatter.field("description", &self.description);
        formatter.field("kms_key_id", &self.kms_key_id);
        formatter.field("secret_versions_to_stages", &self.secret_versions_to_stages);
        formatter.field("created_date", &self.created_date);
        formatter.field("deleted_date", &self.deleted_date);
        formatter.field("arn", &self.arn);
        formatter.field("owning_service", &self.owning_service);
        formatter.field("rotation_lambda_arn", &self.rotation_lambda_arn);
        formatter.field("tags", &self.tags);
        formatter.field("rotation_enabled", &self.rotation_enabled);
        formatter.field("last_accessed_date", &self.last_accessed_date);
        formatter.field("last_changed_date", &self.last_changed_date);
        formatter.field("rotation_rules", &self.rotation_rules);
        formatter.field("name", &self.name);
        formatter.finish()
    }
}
/// See [`SecretListEntry`](crate::model::SecretListEntry)
pub mod secret_list_entry {

    /// A builder for [`SecretListEntry`](crate::model::SecretListEntry)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        last_rotated_date: std::option::Option<smithy_types::Instant>,
        description: std::option::Option<std::string::String>,
        kms_key_id: std::option::Option<std::string::String>,
        secret_versions_to_stages: std::option::Option<
            std::collections::HashMap<std::string::String, std::vec::Vec<std::string::String>>,
        >,
        created_date: std::option::Option<smithy_types::Instant>,
        deleted_date: std::option::Option<smithy_types::Instant>,
        arn: std::option::Option<std::string::String>,
        owning_service: std::option::Option<std::string::String>,
        rotation_lambda_arn: std::option::Option<std::string::String>,
        tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        rotation_enabled: std::option::Option<bool>,
        last_accessed_date: std::option::Option<smithy_types::Instant>,
        last_changed_date: std::option::Option<smithy_types::Instant>,
        rotation_rules: std::option::Option<crate::model::RotationRulesType>,
        name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The last date and time that the rotation process for this secret was invoked.</p>
        pub fn last_rotated_date(mut self, inp: smithy_types::Instant) -> Self {
            self.last_rotated_date = Some(inp);
            self
        }
        pub fn set_last_rotated_date(
            mut self,
            inp: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.last_rotated_date = inp;
            self
        }
        /// <p>The user-provided description of the secret.</p>
        pub fn description(mut self, inp: impl Into<std::string::String>) -> Self {
            self.description = Some(inp.into());
            self
        }
        pub fn set_description(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.description = inp;
            self
        }
        /// <p>The ARN or alias of the AWS KMS customer master key (CMK) used to encrypt the
        /// <code>SecretString</code> and <code>SecretBinary</code> fields in each version of the
        /// secret. If you don't provide a key, then Secrets Manager defaults to encrypting the secret fields with
        /// the default KMS CMK, the key named <code>awssecretsmanager</code>, for this account.</p>
        pub fn kms_key_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.kms_key_id = Some(inp.into());
            self
        }
        pub fn set_kms_key_id(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.kms_key_id = inp;
            self
        }
        pub fn secret_versions_to_stages(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::vec::Vec<std::string::String>>,
        ) -> Self {
            let mut hash_map = self.secret_versions_to_stages.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.secret_versions_to_stages = Some(hash_map);
            self
        }
        pub fn set_secret_versions_to_stages(
            mut self,
            inp: std::option::Option<
                std::collections::HashMap<std::string::String, std::vec::Vec<std::string::String>>,
            >,
        ) -> Self {
            self.secret_versions_to_stages = inp;
            self
        }
        /// <p>The date and time when a secret was created.</p>
        pub fn created_date(mut self, inp: smithy_types::Instant) -> Self {
            self.created_date = Some(inp);
            self
        }
        pub fn set_created_date(mut self, inp: std::option::Option<smithy_types::Instant>) -> Self {
            self.created_date = inp;
            self
        }
        /// <p>The date and time the deletion of the secret occurred. Not present on active secrets. The
        /// secret can be recovered until the number of days in the recovery window has passed, as
        /// specified in the <code>RecoveryWindowInDays</code> parameter of the <a>DeleteSecret</a> operation.</p>
        pub fn deleted_date(mut self, inp: smithy_types::Instant) -> Self {
            self.deleted_date = Some(inp);
            self
        }
        pub fn set_deleted_date(mut self, inp: std::option::Option<smithy_types::Instant>) -> Self {
            self.deleted_date = inp;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the secret.</p>
        /// <p>For more information about ARNs in Secrets Manager, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#iam-resources">Policy Resources</a> in the
        /// <i>AWS Secrets Manager User Guide</i>.</p>
        pub fn arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.arn = Some(inp.into());
            self
        }
        pub fn set_arn(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.arn = inp;
            self
        }
        /// <p>Returns the name of the service that created the secret.</p>
        pub fn owning_service(mut self, inp: impl Into<std::string::String>) -> Self {
            self.owning_service = Some(inp.into());
            self
        }
        pub fn set_owning_service(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.owning_service = inp;
            self
        }
        /// <p>The ARN of an AWS Lambda function invoked by Secrets Manager to rotate and expire the
        /// secret either automatically per the schedule or manually by a call to <a>RotateSecret</a>.</p>
        pub fn rotation_lambda_arn(mut self, inp: impl Into<std::string::String>) -> Self {
            self.rotation_lambda_arn = Some(inp.into());
            self
        }
        pub fn set_rotation_lambda_arn(
            mut self,
            inp: std::option::Option<std::string::String>,
        ) -> Self {
            self.rotation_lambda_arn = inp;
            self
        }
        pub fn tags(mut self, inp: impl Into<crate::model::Tag>) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(inp.into());
            self.tags = Some(v);
            self
        }
        pub fn set_tags(
            mut self,
            inp: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = inp;
            self
        }
        /// <p>Indicates whether automatic, scheduled rotation is enabled for this secret.</p>
        pub fn rotation_enabled(mut self, inp: bool) -> Self {
            self.rotation_enabled = Some(inp);
            self
        }
        pub fn set_rotation_enabled(mut self, inp: std::option::Option<bool>) -> Self {
            self.rotation_enabled = inp;
            self
        }
        /// <p>The last date that this secret was accessed. This value is truncated to midnight of the
        /// date and therefore shows only the date, not the time.</p>
        pub fn last_accessed_date(mut self, inp: smithy_types::Instant) -> Self {
            self.last_accessed_date = Some(inp);
            self
        }
        pub fn set_last_accessed_date(
            mut self,
            inp: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.last_accessed_date = inp;
            self
        }
        /// <p>The last date and time that this secret was modified in any way.</p>
        pub fn last_changed_date(mut self, inp: smithy_types::Instant) -> Self {
            self.last_changed_date = Some(inp);
            self
        }
        pub fn set_last_changed_date(
            mut self,
            inp: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.last_changed_date = inp;
            self
        }
        /// <p>A structure that defines the rotation configuration for the secret.</p>
        pub fn rotation_rules(mut self, inp: crate::model::RotationRulesType) -> Self {
            self.rotation_rules = Some(inp);
            self
        }
        pub fn set_rotation_rules(
            mut self,
            inp: std::option::Option<crate::model::RotationRulesType>,
        ) -> Self {
            self.rotation_rules = inp;
            self
        }
        /// <p>The friendly name of the secret. You can use forward slashes in the name to represent a
        /// path hierarchy. For example, <code>/prod/databases/dbserver1</code> could represent the secret
        /// for a server named <code>dbserver1</code> in the folder <code>databases</code> in the folder
        /// <code>prod</code>. </p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.name = Some(inp.into());
            self
        }
        pub fn set_name(mut self, inp: std::option::Option<std::string::String>) -> Self {
            self.name = inp;
            self
        }
        /// Consumes the builder and constructs a [`SecretListEntry`](crate::model::SecretListEntry)
        pub fn build(self) -> crate::model::SecretListEntry {
            crate::model::SecretListEntry {
                last_rotated_date: self.last_rotated_date,
                description: self.description,
                kms_key_id: self.kms_key_id,
                secret_versions_to_stages: self.secret_versions_to_stages,
                created_date: self.created_date,
                deleted_date: self.deleted_date,
                arn: self.arn,
                owning_service: self.owning_service,
                rotation_lambda_arn: self.rotation_lambda_arn,
                tags: self.tags,
                rotation_enabled: self.rotation_enabled,
                last_accessed_date: self.last_accessed_date,
                last_changed_date: self.last_changed_date,
                rotation_rules: self.rotation_rules,
                name: self.name,
            }
        }
    }
}
impl SecretListEntry {
    /// Creates a new builder-style object to manufacture [`SecretListEntry`](crate::model::SecretListEntry)
    pub fn builder() -> crate::model::secret_list_entry::Builder {
        crate::model::secret_list_entry::Builder::default()
    }
}

/// <p>Allows you to filter your list of secrets.</p>
#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct Filter {
    /// <p>Filters your list of secrets by a specific key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub key: std::option::Option<crate::model::FilterNameStringType>,
    /// <p>Filters your list of secrets by a specific value.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub values: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl std::fmt::Debug for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Filter");
        formatter.field("key", &self.key);
        formatter.field("values", &self.values);
        formatter.finish()
    }
}
/// See [`Filter`](crate::model::Filter)
pub mod filter {

    /// A builder for [`Filter`](crate::model::Filter)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        key: std::option::Option<crate::model::FilterNameStringType>,
        values: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>Filters your list of secrets by a specific key.</p>
        pub fn key(mut self, inp: crate::model::FilterNameStringType) -> Self {
            self.key = Some(inp);
            self
        }
        pub fn set_key(
            mut self,
            inp: std::option::Option<crate::model::FilterNameStringType>,
        ) -> Self {
            self.key = inp;
            self
        }
        pub fn values(mut self, inp: impl Into<std::string::String>) -> Self {
            let mut v = self.values.unwrap_or_default();
            v.push(inp.into());
            self.values = Some(v);
            self
        }
        pub fn set_values(
            mut self,
            inp: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.values = inp;
            self
        }
        /// Consumes the builder and constructs a [`Filter`](crate::model::Filter)
        pub fn build(self) -> crate::model::Filter {
            crate::model::Filter {
                key: self.key,
                values: self.values,
            }
        }
    }
}
impl Filter {
    /// Creates a new builder-style object to manufacture [`Filter`](crate::model::Filter)
    pub fn builder() -> crate::model::filter::Builder {
        crate::model::filter::Builder::default()
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
pub struct FilterNameStringType(String);
impl FilterNameStringType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn values() -> &'static [&'static str] {
        &["all", "description", "name", "tag-key", "tag-value"]
    }
}
impl<T> std::convert::From<T> for FilterNameStringType
where
    T: std::convert::AsRef<str>,
{
    fn from(s: T) -> Self {
        FilterNameStringType(s.as_ref().to_owned())
    }
}

impl serde::Serialize for FilterNameStringType {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FilterNameStringType {
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
pub enum SortOrderType {
    Asc,
    Desc,
    Unknown(String),
}
impl std::convert::From<&str> for SortOrderType {
    fn from(s: &str) -> Self {
        match s {
            "asc" => SortOrderType::Asc,
            "desc" => SortOrderType::Desc,
            other => SortOrderType::Unknown(other.to_owned()),
        }
    }
}

impl std::str::FromStr for SortOrderType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SortOrderType::from(s))
    }
}

impl SortOrderType {
    pub fn as_str(&self) -> &str {
        match self {
            SortOrderType::Asc => "asc",
            SortOrderType::Desc => "desc",
            SortOrderType::Unknown(s) => s.as_ref(),
        }
    }
}
impl AsRef<str> for SortOrderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl serde::Serialize for SortOrderType {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SortOrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = <&str>::deserialize(deserializer)?;
        Ok(Self::from(data))
    }
}
