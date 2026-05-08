use crate::function::StaticFunctionDescription;
use base64::Engine;

use crate::{atomic, error, wrap_xpath_fn};
use xee_xpath_macros::xpath_fn;

// Can't get the macro to work with xs:base64Binary? as the input, working around with a manual cast
#[xpath_fn("bin:decode-string($input as xs:anyAtomicType?) as xs:string?")]
fn decode_string(input: Option<atomic::Atomic>) -> error::Result<Option<String>> {
    match input {
        Some(atomic::Atomic::Binary(binary_type, data)) => match binary_type {
            atomic::BinaryType::Base64 => {
                let s = String::from_utf8(data.to_vec())
                    .expect("Expected valid string in data after casting.");
                Ok(Some(s))
            }
            _ => Err(error::Error::XPTY0004),
        },
        Some(_) => {
            let res = input.map(|arg| arg.cast_to_base64_binary()).transpose()?;
            decode_string(res)
        }
        None => Ok(None),
    }
}

#[xpath_fn("bin:encode-string($input as xs:string?) as xs:base64Binary?")]
fn encode_string(input: Option<&str>) -> error::Result<Option<atomic::Atomic>> {
    match input {
        Some(s) => {
            let encoded_string: atomic::Atomic =
                base64::engine::general_purpose::STANDARD.encode(s).into();
            Ok(Some(encoded_string.cast_to_base64_binary()?))
        }
        _ => Ok(None),
    }
}

#[xpath_fn("fn:base64d($input as xs:anyAtomicType?) as xs:string?")]
fn base64d(input: Option<atomic::Atomic>) -> error::Result<Option<String>> {
    decode_string(input)
}

#[xpath_fn("fn:base64($input as xs:string?) as xs:base64Binary?")]
fn base64e(input: Option<&str>) -> error::Result<Option<atomic::Atomic>> {
    encode_string(input)
}

pub(crate) fn static_function_descriptions() -> Vec<StaticFunctionDescription> {
    vec![wrap_xpath_fn!(decode_string), wrap_xpath_fn!(encode_string), wrap_xpath_fn!(base64d), wrap_xpath_fn!(base64e)]
}
