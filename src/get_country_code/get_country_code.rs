use service_sdk::my_http_server::{HttpRequest, HttpRequestHeaders};

pub trait GetCountryCode {
    fn get_country_code(&self) -> Option<&str>;
}

impl GetCountryCode for HttpRequest {
    fn get_country_code(&self) -> Option<&str> {
        let result = self
            .get_headers()
            .try_get_case_insensitive_as_str("cf-ipcountry");

        if result.is_err() {
            return None;
        }

        result.unwrap()
    }
}
