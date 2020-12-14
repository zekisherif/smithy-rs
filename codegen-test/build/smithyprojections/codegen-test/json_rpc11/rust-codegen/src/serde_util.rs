// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use smithy_types::Blob;
use smithy_types::Document;
use smithy_types::Instant;
pub fn blob_ser<S>(
    _inp: &Blob,
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    _serializer.serialize_str(&::smithy_http::base64::encode(_inp.as_ref()))
}

pub fn blob_deser<'de, D>(_deser: D) -> Result<Blob, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    use ::serde::de::Error;
    use ::serde::Deserialize;
    let data = <&str>::deserialize(_deser)?;
    ::smithy_http::base64::decode(data)
        .map(Blob::new)
        .map_err(|_| D::Error::invalid_value(::serde::de::Unexpected::Str(data), &"valid base64"))
}

pub fn instant_epoch_seconds_ser<S>(
    _inp: &Instant,
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    _serializer.serialize_i64(_inp.epoch_seconds())
}

pub fn instant_epoch_seconds_deser<'de, D>(_deser: D) -> Result<Instant, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    use ::serde::Deserialize;

    let ts = f64::deserialize(_deser)?;
    Ok(Instant::from_fractional_seconds(
        ts.floor() as i64,
        ts - ts.floor(),
    ))
}

pub fn stdoptionoptionblob_ser<S>(
    _inp: &::std::option::Option<Blob>,
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    match _inp {
        Some(blob) => _serializer.serialize_str(&::smithy_http::base64::encode(blob.as_ref())),
        None => _serializer.serialize_none(),
    }
}

pub fn stdoptionoptionblob_deser<'de, D>(_deser: D) -> Result<::std::option::Option<Blob>, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    use ::serde::de::Error;
    use ::serde::Deserialize;
    Option::<&str>::deserialize(_deser)?
        .map(|data| {
            ::smithy_http::base64::decode(data)
                .map(Blob::new)
                .map_err(|_| {
                    D::Error::invalid_value(::serde::de::Unexpected::Str(data), &"valid base64")
                })
        })
        .transpose()
}

pub fn stdoptionoptioninstant_http_date_ser<S>(
    _inp: &::std::option::Option<Instant>,
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    match _inp {
        Some(ts) => _serializer.serialize_some(&ts.fmt(::smithy_types::instant::Format::HttpDate)),
        None => _serializer.serialize_none(),
    }
}

pub fn stdoptionoptioninstant_http_date_deser<'de, D>(
    _deser: D,
) -> Result<::std::option::Option<Instant>, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    todo!()
}

pub fn stdoptionoptioninstant_date_time_ser<S>(
    _inp: &::std::option::Option<Instant>,
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    match _inp {
        Some(ts) => _serializer.serialize_some(&ts.fmt(::smithy_types::instant::Format::DateTime)),
        None => _serializer.serialize_none(),
    }
}

pub fn stdoptionoptioninstant_date_time_deser<'de, D>(
    _deser: D,
) -> Result<::std::option::Option<Instant>, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    todo!()
}

pub fn stdoptionoptioninstant_epoch_seconds_ser<S>(
    _inp: &::std::option::Option<Instant>,
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    match _inp {
        Some(ts) => _serializer.serialize_some(&ts.epoch_seconds()),
        None => _serializer.serialize_none(),
    }
}

pub fn stdoptionoptioninstant_epoch_seconds_deser<'de, D>(
    _deser: D,
) -> Result<::std::option::Option<Instant>, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    use ::serde::Deserialize;

    let ts_opt = Option::<f64>::deserialize(_deser)?;
    Ok(ts_opt.map(|ts| Instant::from_fractional_seconds(ts.floor() as i64, ts - ts.floor())))
}

pub fn document_ser<S>(
    _inp: &Document,
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    use ::serde::Serialize;
    crate::doc_json::SerDoc(_inp).serialize(_serializer)
}

pub fn document_deser<'de, D>(_deser: D) -> Result<Document, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    todo!()
}
