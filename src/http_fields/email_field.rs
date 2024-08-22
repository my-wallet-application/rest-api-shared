service_sdk::macros::use_my_http_server!();
use email_address::EmailAddress;
use rust_extensions::StrOrString;

#[http_input_field]
pub struct EmailHttpField(String);

fn process_value(src: &str) -> Result<StrOrString, HttpFailResult> {
    let email = src.trim().to_lowercase();

    if !EmailAddress::is_valid(&email) {
        return Err(HttpFailResult::as_validation_error(
            "Email is not valid".to_string(),
        ));
    }

    Ok(StrOrString::create_as_string(email))
}
