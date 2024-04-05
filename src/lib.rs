//! An incomplete set of data transfer objects (DTOs) and errors for
//! Synology DSM APIs.
//! 
//! ## Features
//! 
//! By default, all of the following features are enabled.
//! 
//! * `dto` - provides DTOs for various JSON responses produced by
//!   Synology DSM APIs
//! * `error` - provides enums implementing `Error` and `Display` traits,
//!   and `From`/`TryFrom` for `u16` to convert from error codes returned
//!   by the API to enum values
//! * `serde` - adds `Serialize` and `Deserialize` trait implementations
//!   to DTOs (adds dependency on `serde` library).

pub mod auth;
#[cfg(feature = "error")]
pub mod error;
pub mod foto;
pub mod foto_team;

#[cfg(feature = "dto")]
pub mod dto {
    //! Generic types applicable to all sub-APIs

    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct ApiResponse<D> {
        pub success: bool,
        pub data: Option<D>,
        pub error: Option<Error>,
    }

    pub type ApiResponseWithList<T> = ApiResponse<List<T>>;

    #[derive(Debug, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct List<T> {
        pub list: Vec<T>,
    }

    #[derive(Debug, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Error {
        pub code: u16,
    }
}
