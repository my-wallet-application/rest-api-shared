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

    #[http_enum_case(id="-1"; description="Invalid username or password")]
    InvalidUserNameOrPassword = -1,

    #[http_enum_case(id="-2"; description="User exists")]
    UserExists = -2,

    #[http_enum_case(id="-3"; description="User not found")]
    UserNotFound = -3,

    #[http_enum_case(id="-4"; description="Old password is wrong")]
    OldPasswordIsWrong = -4,

    #[http_enum_case(id="-5"; description="Wrong file extension")]
    WrongFileExtension = -5,

    #[http_enum_case(id="-6"; description="Crypto deposit is not supported")]
    CryptoDepositIsNotSupported = -6,

    #[http_enum_case(id="-7"; description="Personal data is not valid")]
    PersonalDataNotValid = -7,

    #[http_enum_case(id="-8"; description="Not enough funds")]
    NotEnoughFunds = -8,

    #[http_enum_case(id="-9"; description="AccessTokenExpired")]
    AccessTokenExpired = -9,

    #[http_enum_case(id="-10"; description="TechnicalError")]
    TechnicalError = -10,

    #[http_enum_case(id="-11"; description="CountryRestriction")]
    CountryIsRestricted = -11,

    #[http_enum_case(id="-12"; description="Swap quote is expired")]
    SwapQuoteIsExpired = -12,

    #[http_enum_case(id="-13"; description="No liquidity")]
    NoLiquidity = -13,

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
            ApiResultStatus::FileNotFound => 200,
            ApiResultStatus::PersonalDataNotValid => 200,

            ApiResultStatus::AccessTokenExpired => 401,

            ApiResultStatus::TechnicalError => 200,
            ApiResultStatus::CountryIsRestricted => 200,
            ApiResultStatus::ForceUpdateIsRequired => 200,
            ApiResultStatus::NotEnoughFunds => 200,
            ApiResultStatus::SwapQuoteIsExpired => 200,
            ApiResultStatus::NoLiquidity => 200,
            ApiResultStatus::AccessClaimRequired => 403,
            ApiResultStatus::CryptoDepositIsNotSupported => 200,
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
        ApiResultStatus::FileNotFound => false,
        ApiResultStatus::PersonalDataNotValid => false,

        ApiResultStatus::AccessTokenExpired => false,
        ApiResultStatus::TechnicalError => true,
        ApiResultStatus::CountryIsRestricted => false,

        ApiResultStatus::ForceUpdateIsRequired => false,
        ApiResultStatus::NotEnoughFunds => false,
        ApiResultStatus::SwapQuoteIsExpired => false,
        ApiResultStatus::NoLiquidity => false,
        ApiResultStatus::AccessClaimRequired => false,
        ApiResultStatus::CryptoDepositIsNotSupported => false,
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
