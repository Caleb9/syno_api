//! SYNO.API.Auth

pub const API: &str = "SYNO.API.Auth";

#[cfg(feature = "dto")]
pub mod dto {
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Login {
        pub did: String,
        pub sid: String,
    }
}

#[cfg(feature = "error")]
pub mod error {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Copy, Clone)]
    pub enum AuthError {
        NoSuchAccountOrIncorrectPassword = 400,
        DisabledAccount = 401,
        PermissionDenied = 402,
        MfaCodeRequired = 403,
        InvalidMfaCode = 404,
        EnforceAuthWithMfa = 406,
        BlockedIpSource = 407,
        ExpiredPasswordCannotChange = 408,
        ExpiredPassword = 409,
        PasswordMustBeChanged = 410,
    }

    impl Display for AuthError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                AuthError::NoSuchAccountOrIncorrectPassword => {
                    "no such account or incorrect password"
                }
                AuthError::DisabledAccount => "disabled user account",
                AuthError::PermissionDenied => "permission denied",
                AuthError::MfaCodeRequired => "2FA OTP code required",
                AuthError::InvalidMfaCode => "invalid OTP code",
                AuthError::EnforceAuthWithMfa => "",
                AuthError::BlockedIpSource => "blocked IP source",
                AuthError::ExpiredPasswordCannotChange | AuthError::ExpiredPassword => {
                    "expired password"
                }
                AuthError::PasswordMustBeChanged => "password must be changed",
            };
            write!(f, "{s}")
        }
    }

    impl Error for AuthError {}

    impl TryFrom<u16> for AuthError {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            let auth_err = match value {
                400 => AuthError::NoSuchAccountOrIncorrectPassword,
                401 => AuthError::DisabledAccount,
                402 => AuthError::PermissionDenied,
                403 => AuthError::MfaCodeRequired,
                404 => AuthError::InvalidMfaCode,
                406 => AuthError::EnforceAuthWithMfa,
                407 => AuthError::BlockedIpSource,
                408 => AuthError::ExpiredPasswordCannotChange,
                409 => AuthError::ExpiredPassword,
                410 => AuthError::PasswordMustBeChanged,

                other => return Err(other),
            };
            Ok(auth_err)
        }
    }
}
