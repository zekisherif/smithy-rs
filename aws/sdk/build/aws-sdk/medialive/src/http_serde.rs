// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_describe_input_device_thumbnail_body(
    body: &mut smithy_http::body::SdkBody,
) -> Result<smithy_http::byte_stream::ByteStream, crate::error::DescribeInputDeviceThumbnailError> {
    // replace the body with an empty body
    let body = std::mem::replace(body, smithy_http::body::SdkBody::taken());
    Ok(smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_header_describe_input_device_thumbnail_content_length(
    header_map: &http::HeaderMap,
) -> Result<i64, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Length").iter();
    let var_1: Vec<i64> = smithy_http::header::read_many(headers)?;
    if var_1.len() > 1 {
        return Err(smithy_http::header::ParseError);
    }
    let mut var_1 = var_1;
    match var_1.pop() {
        None => Ok(Default::default()),
        Some(item) => Ok(item),
    }
}

pub fn deser_header_describe_input_device_thumbnail_content_type(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<crate::model::ContentType>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Type").iter();
    let var_2: Vec<crate::model::ContentType> = smithy_http::header::read_many(headers)?;
    if var_2.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub fn deser_header_describe_input_device_thumbnail_e_tag(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("ETag").iter();
    let var_3: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_3.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub fn deser_header_describe_input_device_thumbnail_last_modified(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<smithy_types::Instant>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Last-Modified").iter();
    let var_4: Vec<smithy_types::Instant> =
        smithy_http::header::many_dates(headers, smithy_types::instant::Format::HttpDate)?;
    if var_4.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}
