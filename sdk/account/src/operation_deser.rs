// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_alternate_contact_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DeleteAlternateContactOutput,
    crate::error::DeleteAlternateContactError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DeleteAlternateContactError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::DeleteAlternateContactError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DeleteAlternateContactError {
            meta: generic,
            kind: crate::error::DeleteAlternateContactErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::DeleteAlternateContactError {
            meta: generic,
            kind: crate::error::DeleteAlternateContactErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::DeleteAlternateContactError {
            meta: generic,
            kind: crate::error::DeleteAlternateContactErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "TooManyRequestsException" => crate::error::DeleteAlternateContactError {
            meta: generic,
            kind: crate::error::DeleteAlternateContactErrorKind::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::DeleteAlternateContactError {
            meta: generic,
            kind: crate::error::DeleteAlternateContactErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DeleteAlternateContactError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_alternate_contact_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DeleteAlternateContactOutput,
    crate::error::DeleteAlternateContactError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_alternate_contact_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_alternate_contact_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetAlternateContactOutput,
    crate::error::GetAlternateContactError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetAlternateContactError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetAlternateContactError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::GetAlternateContactError {
            meta: generic,
            kind: crate::error::GetAlternateContactErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::GetAlternateContactError {
            meta: generic,
            kind: crate::error::GetAlternateContactErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::GetAlternateContactError {
            meta: generic,
            kind: crate::error::GetAlternateContactErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "TooManyRequestsException" => crate::error::GetAlternateContactError {
            meta: generic,
            kind: crate::error::GetAlternateContactErrorKind::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::GetAlternateContactError {
            meta: generic,
            kind: crate::error::GetAlternateContactErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetAlternateContactError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_alternate_contact_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetAlternateContactOutput,
    crate::error::GetAlternateContactError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_alternate_contact_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_alternate_contact(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetAlternateContactError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_put_alternate_contact_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::PutAlternateContactOutput,
    crate::error::PutAlternateContactError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::PutAlternateContactError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::PutAlternateContactError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::PutAlternateContactError {
            meta: generic,
            kind: crate::error::PutAlternateContactErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::PutAlternateContactError {
            meta: generic,
            kind: crate::error::PutAlternateContactErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "TooManyRequestsException" => crate::error::PutAlternateContactError {
            meta: generic,
            kind: crate::error::PutAlternateContactErrorKind::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::PutAlternateContactError {
            meta: generic,
            kind: crate::error::PutAlternateContactErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutAlternateContactError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::PutAlternateContactError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_put_alternate_contact_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::PutAlternateContactOutput,
    crate::error::PutAlternateContactError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_alternate_contact_output::Builder::default();
        let _ = response;
        output.build()
    })
}