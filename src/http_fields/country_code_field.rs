use rust_common::country_code::CountryCode;
use rust_extensions::StrOrString;

service_sdk::macros::use_my_http_server!();

#[http_input_field]
pub struct CountryCodeHttpField(String);

fn process_value(src: &str) -> Result<StrOrString, HttpFailResult> {
    let value = src.trim().to_uppercase();

    let parsed = CountryCode::parse(value.as_str());

    if parsed.is_err() {
        return Err(HttpFailResult::as_validation_error(format!(
            "Country code {} is not valid",
            src
        )));
    }

    Ok(StrOrString::create_as_string(value))
}

impl Into<CountryCode> for CountryCodeHttpField {
    fn into(self) -> CountryCode {
        CountryCode::parse(self.0.as_str()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_iso3() {
        let processed = CountryCodeHttpField::new("USA").unwrap();
        match processed.into() {
            CountryCode::USA => {}
            _ => panic!("Invalid country code"),
        }
    }

    #[test]
    fn test_correct_iso3_lowercase() {
        let processed = CountryCodeHttpField::new("usa").unwrap();
        match processed.into() {
            CountryCode::USA => {}
            _ => panic!("Invalid country code"),
        }
    }

    #[test]
    fn test_correct_iso2_lowercase() {
        let processed = CountryCodeHttpField::new("us").unwrap();
        match processed.into() {
            CountryCode::USA => {}
            _ => panic!("Invalid country code"),
        }
    }

    #[test]
    fn test_non_correct_iso_code() {
        let processed = CountryCodeHttpField::new("GGG");
        assert_eq!(true, processed.is_err());

        let processed = processed.unwrap_err();
        assert_eq!(processed.status_code, 400);
    }
}
