//! enums implementing `Error` for various error codes returned by DSM API

use crate::auth::error::AuthError;
use crate::foto::error::PhotoError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Error {
    Api(ApiError),     // 100-150
    Auth(AuthError),   // 400-404, 406-410
    Photo(PhotoError), // 642
    Unknown(Code),
}

/// Taken from [DSM_Login_Web_API_Guide](https://kb.synology.com/en-us/DG/DSM_Login_Web_API_Guide/)
#[derive(Debug, Copy, Clone)]
pub enum ApiError {
    Unknown,                             // 100
    InvalidParameter,                    // 101
    InvalidApi,                          // 102
    InvalidMethod,                       // 103
    InvalidVersion,                      // 104
    InvalidSessionPermission,            // 105
    SessionTimeout,                      // 106
    SessionInterruptedByDuplicatedLogin, // 107
    FileUploadFailed,                    // 108
    NetUnstableOrSystemBusy(u16),        // 109-111, 117, 118
    LostParamsForApi,                    // 114
    FileUploadNotAllowed,                // 115
    NotAllowedForDemoSite,               // 116 (?)
    InvalidSession,                      // 119
    RequestLoginIpMismatch,              // 150
                                         // 112, 113, 120-149 - preserve for other purpose (?)
}

#[derive(Debug, Copy, Clone)]
pub struct Code(u16);

impl std::error::Error for Error {}
impl std::error::Error for ApiError {}
impl std::error::Error for Code {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Api(e) => write!(f, "{e}"),
            Error::Auth(e) => write!(f, "{e}"),
            Error::Photo(e) => write!(f, "{e}"),
            Error::Unknown(code) => write!(f, "{code}"),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ApiError::Unknown => "unknown error",
            ApiError::InvalidParameter => "no parameter of API, method or version",
            ApiError::InvalidApi => "the requested API does not exist",
            ApiError::InvalidMethod => "the method does not exist",
            ApiError::InvalidVersion => "the requested version does not support the functionality",
            ApiError::InvalidSessionPermission => "the logged in session does not have permission",
            ApiError::SessionTimeout => "session timeout",
            ApiError::SessionInterruptedByDuplicatedLogin => {
                "session interrupted by duplicated login"
            }
            ApiError::FileUploadFailed => "failed to upload the file",
            ApiError::NetUnstableOrSystemBusy(_) => {
                "the network connection is unstable or the system is busy"
            }
            ApiError::LostParamsForApi => "lost parameters for this API",
            ApiError::FileUploadNotAllowed => "not allowed to upload a file",
            ApiError::NotAllowedForDemoSite => "not allowed to perform for a demo site",
            ApiError::InvalidSession => "invalid session",
            ApiError::RequestLoginIpMismatch => "request source IP does not match the login IP",
        };
        write!(f, "{s}")
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for Error {
    fn from(value: u16) -> Self {
        if let Ok(api_error) = ApiError::try_from(value) {
            Error::Api(api_error)
        } else if let Ok(auth_error) = AuthError::try_from(value) {
            Error::Auth(auth_error)
        } else if let Ok(photo_error) = PhotoError::try_from(value) {
            Error::Photo(photo_error)
        } else {
            Error::Unknown(Code(value))
        }
    }
}

impl TryFrom<u16> for ApiError {
    type Error = Code;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let api_error = match value {
            100 => ApiError::Unknown,
            101 => ApiError::InvalidParameter,
            102 => ApiError::InvalidApi,
            103 => ApiError::InvalidMethod,
            104 => ApiError::InvalidVersion,
            105 => ApiError::InvalidSessionPermission,
            106 => ApiError::SessionTimeout,
            107 => ApiError::SessionInterruptedByDuplicatedLogin,
            108 => ApiError::FileUploadFailed,
            code @ (109..=111 | 117 | 118) => ApiError::NetUnstableOrSystemBusy(code),
            114 => ApiError::LostParamsForApi,
            115 => ApiError::FileUploadNotAllowed,
            116 => ApiError::NotAllowedForDemoSite,
            119 => ApiError::InvalidSession,
            150 => ApiError::RequestLoginIpMismatch,
            code => return Err(Code(code)),
        };
        Ok(api_error)
    }
}

#[cfg(test)]
mod test {
    use crate::error::ApiError;

    #[test]
    fn from_api_error_returns_valid_u16() {
        let unknown: u16 = ApiError::Unknown.into();
        let lost_params: u16 = ApiError::LostParamsForApi.into();
        assert_eq!(unknown, 100);
        assert_eq!(lost_params, 114);
    }
}
