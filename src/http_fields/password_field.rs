use rust_extensions::StrOrString;

service_sdk::macros::use_my_http_server!();

const PASSWORD_LENGTH: usize = 6;

#[http_input_field]
pub struct PasswordHttpField(String);

fn process_value(src: &str) -> Result<StrOrString, HttpFailResult> {
    if src.len() < PASSWORD_LENGTH {
        return Err(HttpFailResult::as_validation_error(format!(
            "Password must have at least {PASSWORD_LENGTH} chars",
        )));
    }

    let mut digits = 0;

    for c in src.chars() {
        if c.is_numeric() {
            digits += 1;
        }
    }

    if digits == 0 {
        return Err(HttpFailResult::as_validation_error(
            "Password must have at least one digit".to_string(),
        ));
    }

    Ok(StrOrString::create_as_str(src))
}
