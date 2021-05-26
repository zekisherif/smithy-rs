// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<smithy_types::Error, serde_json::Error> {
    let body =
        serde_json::from_slice(response.body().as_ref()).unwrap_or_else(|_| serde_json::json!({}));
    Ok(crate::aws_json_errors::parse_generic_error(
        &response, &body,
    ))
}

pub fn invalid_parameter_exception(
    inp: &[u8],
    mut builder: crate::error::invalid_parameter_error::Builder,
) -> Result<crate::error::invalid_parameter_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::InvalidParameterError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_message(parsed_body.message);
    builder = builder.set_parameter_name(parsed_body.parameter_name);
    Ok(builder)
}

pub fn resource_not_found_exception(
    inp: &[u8],
    mut builder: crate::error::resource_not_found_error::Builder,
) -> Result<crate::error::resource_not_found_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::ResourceNotFoundError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_message(parsed_body.message);
    builder = builder.set_resource_type(parsed_body.resource_type);
    builder = builder.set_resource_name(parsed_body.resource_name);
    Ok(builder)
}

pub fn resource_precondition_not_met_exception(
    inp: &[u8],
    mut builder: crate::error::resource_precondition_not_met_error::Builder,
) -> Result<crate::error::resource_precondition_not_met_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::ResourcePreconditionNotMetError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_message(parsed_body.message);
    builder = builder.set_resource_type(parsed_body.resource_type);
    builder = builder.set_resource_name(parsed_body.resource_name);
    Ok(builder)
}

pub fn cancel_journal_kinesis_stream_deser_operation(
    inp: &[u8],
    mut builder: crate::output::cancel_journal_kinesis_stream_output::Builder,
) -> Result<crate::output::cancel_journal_kinesis_stream_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CancelJournalKinesisStreamOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_stream_id(parsed_body.stream_id);
    Ok(builder)
}

pub fn limit_exceeded_exception(
    inp: &[u8],
    mut builder: crate::error::limit_exceeded_error::Builder,
) -> Result<crate::error::limit_exceeded_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::LimitExceededError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_message(parsed_body.message);
    builder = builder.set_resource_type(parsed_body.resource_type);
    Ok(builder)
}

pub fn resource_already_exists_exception(
    inp: &[u8],
    mut builder: crate::error::resource_already_exists_error::Builder,
) -> Result<crate::error::resource_already_exists_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::ResourceAlreadyExistsError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_message(parsed_body.message);
    builder = builder.set_resource_type(parsed_body.resource_type);
    builder = builder.set_resource_name(parsed_body.resource_name);
    Ok(builder)
}

pub fn resource_in_use_exception(
    inp: &[u8],
    mut builder: crate::error::resource_in_use_error::Builder,
) -> Result<crate::error::resource_in_use_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::ResourceInUseError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_message(parsed_body.message);
    builder = builder.set_resource_type(parsed_body.resource_type);
    builder = builder.set_resource_name(parsed_body.resource_name);
    Ok(builder)
}

pub fn create_ledger_deser_operation(
    inp: &[u8],
    mut builder: crate::output::create_ledger_output::Builder,
) -> Result<crate::output::create_ledger_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CreateLedgerOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_name(parsed_body.name);
    builder = builder.set_arn(parsed_body.arn);
    builder = builder.set_state(parsed_body.state);
    builder = builder.set_creation_date_time(parsed_body.creation_date_time);
    builder = builder.set_deletion_protection(parsed_body.deletion_protection);
    Ok(builder)
}

pub fn describe_journal_kinesis_stream_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_journal_kinesis_stream_output::Builder,
) -> Result<crate::output::describe_journal_kinesis_stream_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeJournalKinesisStreamOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_stream(parsed_body.stream);
    Ok(builder)
}

pub fn describe_journal_s3_export_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_journal_s3_export_output::Builder,
) -> Result<crate::output::describe_journal_s3_export_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeJournalS3ExportOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_export_description(parsed_body.export_description);
    Ok(builder)
}

pub fn describe_ledger_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_ledger_output::Builder,
) -> Result<crate::output::describe_ledger_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeLedgerOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_name(parsed_body.name);
    builder = builder.set_arn(parsed_body.arn);
    builder = builder.set_state(parsed_body.state);
    builder = builder.set_creation_date_time(parsed_body.creation_date_time);
    builder = builder.set_deletion_protection(parsed_body.deletion_protection);
    Ok(builder)
}

pub fn export_journal_to_s3_deser_operation(
    inp: &[u8],
    mut builder: crate::output::export_journal_to_s3_output::Builder,
) -> Result<crate::output::export_journal_to_s3_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ExportJournalToS3OutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_export_id(parsed_body.export_id);
    Ok(builder)
}

pub fn get_block_deser_operation(
    inp: &[u8],
    mut builder: crate::output::get_block_output::Builder,
) -> Result<crate::output::get_block_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetBlockOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_block(parsed_body.block);
    builder = builder.set_proof(parsed_body.proof);
    Ok(builder)
}

pub fn get_digest_deser_operation(
    inp: &[u8],
    mut builder: crate::output::get_digest_output::Builder,
) -> Result<crate::output::get_digest_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetDigestOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_digest(parsed_body.digest);
    builder = builder.set_digest_tip_address(parsed_body.digest_tip_address);
    Ok(builder)
}

pub fn get_revision_deser_operation(
    inp: &[u8],
    mut builder: crate::output::get_revision_output::Builder,
) -> Result<crate::output::get_revision_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetRevisionOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_proof(parsed_body.proof);
    builder = builder.set_revision(parsed_body.revision);
    Ok(builder)
}

pub fn list_journal_kinesis_streams_for_ledger_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_journal_kinesis_streams_for_ledger_output::Builder,
) -> Result<crate::output::list_journal_kinesis_streams_for_ledger_output::Builder, serde_json::Error>
{
    let parsed_body: crate::serializer::ListJournalKinesisStreamsForLedgerOutputBody =
        if inp.is_empty() {
            // To enable JSON parsing to succeed, replace an empty body
            // with an empty JSON body. If a member was required, it will fail slightly later
            // during the operation construction phase when a required field was missing.
            serde_json::from_slice(b"{}")?
        } else {
            serde_json::from_slice(inp)?
        };

    builder = builder.set_streams(parsed_body.streams);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_journal_s3_exports_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_journal_s3_exports_output::Builder,
) -> Result<crate::output::list_journal_s3_exports_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListJournalS3ExportsOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_journal_s3_exports(parsed_body.journal_s3_exports);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_journal_s3_exports_for_ledger_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_journal_s3_exports_for_ledger_output::Builder,
) -> Result<crate::output::list_journal_s3_exports_for_ledger_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListJournalS3ExportsForLedgerOutputBody = if inp.is_empty()
    {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_journal_s3_exports(parsed_body.journal_s3_exports);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_ledgers_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_ledgers_output::Builder,
) -> Result<crate::output::list_ledgers_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListLedgersOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_ledgers(parsed_body.ledgers);
    builder = builder.set_next_token(parsed_body.next_token);
    Ok(builder)
}

pub fn list_tags_for_resource_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_tags_for_resource_output::Builder,
) -> Result<crate::output::list_tags_for_resource_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListTagsForResourceOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_tags(parsed_body.tags);
    Ok(builder)
}

pub fn stream_journal_to_kinesis_deser_operation(
    inp: &[u8],
    mut builder: crate::output::stream_journal_to_kinesis_output::Builder,
) -> Result<crate::output::stream_journal_to_kinesis_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::StreamJournalToKinesisOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_stream_id(parsed_body.stream_id);
    Ok(builder)
}

pub fn update_ledger_deser_operation(
    inp: &[u8],
    mut builder: crate::output::update_ledger_output::Builder,
) -> Result<crate::output::update_ledger_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::UpdateLedgerOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };

    builder = builder.set_name(parsed_body.name);
    builder = builder.set_arn(parsed_body.arn);
    builder = builder.set_state(parsed_body.state);
    builder = builder.set_creation_date_time(parsed_body.creation_date_time);
    builder = builder.set_deletion_protection(parsed_body.deletion_protection);
    Ok(builder)
}
