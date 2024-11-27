use serde::Serialize;
use serde_repr::*;
use service_sdk::my_http_server;
use service_sdk::my_http_server::controllers::documentation::DataTypeProvider;
use service_sdk::my_http_server::macros::{MyHttpIntegerEnum, MyHttpObjectStructure};
use service_sdk::my_http_server::*;

#[derive(Serialize_repr, Deserialize_repr, MyHttpIntegerEnum, Debug, Clone, Copy)]
#[repr(i16)]
pub enum ApiResultStatus {
    #[http_enum_case(id="0"; description="Operations was successful")]
    Ok,

    #[http_enum_case(id="-1"; description="AccessTokenInvalid")]
    TokenIsInvalid = -1,

    #[http_enum_case(id="-2"; description="AccessTokenExpired")]
    AccessTokenExpired = -2,

    #[http_enum_case(id="-3"; description="Invalid username or password")]
    InvalidUserNameOrPassword = -3,

    #[http_enum_case(id="-4"; description="User exists")]
    UserExists = -4,

    #[http_enum_case(id="-5"; description="User not found")]
    UserNotFound = -5,

    #[http_enum_case(id="-6"; description="Old password is wrong")]
    OldPasswordIsWrong = -6,

    #[http_enum_case(id="-7"; description="Wrong file extension")]
    WrongFileExtension = -7,

    #[http_enum_case(id="-8"; description="Crypto deposit is not supported")]
    CryptoDepositIsNotSupported = -8,

    #[http_enum_case(id="-9"; description="Personal data is not valid")]
    PersonalDataNotValid = -9,

    #[http_enum_case(id="-10"; description="Not enough funds")]
    NotEnoughFunds = -10,

    #[http_enum_case(id="-11"; description="CountryRestriction")]
    CountryIsRestricted = -11,

    #[http_enum_case(id="-12"; description="Exchange quote is expired")]
    ExchangeQuoteIsExpired = -12,

    #[http_enum_case(id="-13"; description="No liquidity")]
    NoLiquidity = -13,

    #[http_enum_case(id="-14"; description="Recaptcha verification fail")]
    RecaptchaVerificationFail = -14,

    #[http_enum_case(id="-15"; description="Exchange between assets is disabled")]
    ExchangeBetweenAssetsIsDisabled = -15,

    #[http_enum_case(id="-998"; description="Access claim required")]
    AccessClaimRequired = -998,

    #[http_enum_case(id="-999"; description="Force Update required")]
    ForceUpdateIsRequired = -999,
}

impl ApiResultStatus {
    pub fn get_status_code(&self) -> u16 {
        match self {
            ApiResultStatus::Ok => 200,
            ApiResultStatus::InvalidUserNameOrPassword => 200,
            ApiResultStatus::UserExists => 200,
            ApiResultStatus::UserNotFound => 200,
            ApiResultStatus::OldPasswordIsWrong => 200,
            ApiResultStatus::WrongFileExtension => 200,
            ApiResultStatus::PersonalDataNotValid => 200,

            ApiResultStatus::AccessTokenExpired => 401,
            ApiResultStatus::TokenIsInvalid => 401,

            ApiResultStatus::RecaptchaVerificationFail => 401,
            ApiResultStatus::CountryIsRestricted => 200,
            ApiResultStatus::ForceUpdateIsRequired => 200,
            ApiResultStatus::NotEnoughFunds => 200,
            ApiResultStatus::ExchangeQuoteIsExpired => 200,
            ApiResultStatus::NoLiquidity => 200,
            ApiResultStatus::AccessClaimRequired => 403,
            ApiResultStatus::CryptoDepositIsNotSupported => 200,
            ApiResultStatus::ExchangeBetweenAssetsIsDisabled => 200,
        }
    }
}

#[derive(Serialize, MyHttpObjectStructure)]
pub struct ApiHttpResult {
    pub status: ApiResultStatus,
}

impl Into<HttpFailResult> for ApiHttpResult {
    fn into(self) -> HttpFailResult {
        self.status.into()
    }
}

impl Into<HttpFailResult> for ApiResultStatus {
    fn into(self) -> HttpFailResult {
        let status_code = self.get_status_code();
        let result = ApiHttpResult { status: self };

        let write_to_telemetry = write_to_telemetry(&self);

        HttpFailResult::new(
            my_http_server::WebContentType::Json,
            status_code,
            serde_json::to_vec(&result).unwrap(),
            write_to_telemetry,
            write_to_telemetry,
        )
    }
}

#[derive(Serialize, MyHttpObjectStructure)]
pub struct ApiHttpResultWithData<TData: Serialize + DataTypeProvider> {
    pub status: ApiResultStatus,
    pub data: Option<TData>,
}

impl<TData: Serialize + DataTypeProvider> Into<HttpFailResult> for ApiHttpResultWithData<TData> {
    fn into(self) -> HttpFailResult {
        let status_code = self.status.get_status_code();

        let write_to_telemetry = write_to_telemetry(&self.status);

        HttpFailResult::new(
            my_http_server::WebContentType::Json,
            status_code,
            serde_json::to_vec(&self).unwrap(),
            write_to_telemetry,
            write_to_telemetry,
        )
    }
}

fn write_to_telemetry(from: &ApiResultStatus) -> bool {
    match from {
        ApiResultStatus::Ok => true,
        ApiResultStatus::InvalidUserNameOrPassword => false,
        ApiResultStatus::UserExists => false,
        ApiResultStatus::UserNotFound => false,
        ApiResultStatus::OldPasswordIsWrong => false,
        ApiResultStatus::WrongFileExtension => false,
        ApiResultStatus::PersonalDataNotValid => false,

        ApiResultStatus::AccessTokenExpired => false,
        ApiResultStatus::RecaptchaVerificationFail => false,
        ApiResultStatus::CountryIsRestricted => false,

        ApiResultStatus::ForceUpdateIsRequired => false,
        ApiResultStatus::NotEnoughFunds => false,
        ApiResultStatus::ExchangeQuoteIsExpired => false,
        ApiResultStatus::NoLiquidity => false,
        ApiResultStatus::AccessClaimRequired => false,
        ApiResultStatus::CryptoDepositIsNotSupported => false,
        ApiResultStatus::TokenIsInvalid => false,
        ApiResultStatus::ExchangeBetweenAssetsIsDisabled => true,
    }
}

#[cfg(test)]
mod test {
    use super::ApiResultStatus;
    use serde::Serialize;
    #[derive(Serialize, Debug)]
    pub struct TestStruct {
        result: ApiResultStatus,
    }

    #[test]
    pub fn test_result_deserialization() {
        let test_struct = TestStruct {
            result: ApiResultStatus::AccessTokenExpired,
        };

        let result = serde_json::to_string(&test_struct).unwrap();

        println!("{}", result);
    }
}
