//! SYNO.Foto (Synology Photos)
//!
//! Note that Synology Photos API is not public, so official documentation is not available.
//! All the contents of this module have been reverse-engineered from Synology Photos web app.

pub mod search {
    pub const API: &str = "SYNO.Foto.Search.Search";

    #[cfg(feature = "dto")]
    pub mod dto {
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Eq, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
        pub struct Search {
            pub id: u32,
            pub r#type: String,
            pub name: String,
            pub passphrase: String,
        }
    }
}

pub mod browse {
    pub mod album {
        pub const API: &str = "SYNO.Foto.Browse.Album";

        #[cfg(feature = "dto")]
        pub mod dto {
            #[cfg(feature = "serde")]
            use serde::{Deserialize, Serialize};

            #[derive(Debug, Eq, PartialEq, Hash)]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            pub struct Album {
                pub id: u32,
                pub r#type: String,
                pub item_count: u32,
                pub name: String,
                pub owner_user_id: u32,
                pub passphrase: String,
                pub shared: bool,
                pub temporary_shared: bool,
                pub sort_by: String,
                pub sort_direction: String,
                pub create_time: u64,
                pub start_time: u64,
                pub end_time: u64,
                pub freeze_album: bool,
                pub version: u32,
            }
        }
    }

    pub mod item {
        pub const API: &str = "SYNO.Foto.Browse.Item";

        #[cfg(feature = "dto")]
        pub mod dto {
            #[cfg(feature = "serde")]
            use serde::{Deserialize, Serialize};

            #[derive(Debug, Eq, PartialEq, Hash)]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            pub struct Item {
                pub id: u32,
                pub r#type: String,
                pub filename: String,
                pub filesize: u32,
                pub time: u64,
                pub indexed_time: u64,
                pub owner_user_id: u32,
                pub folder_id: u32,
            }
        }
    }

    pub mod folder {
        pub const API: &str = "SYNO.Foto.Browse.Folder";

        #[cfg(feature = "dto")]
        pub mod dto {
            #[cfg(feature = "serde")]
            use serde::{Deserialize, Serialize};

            #[derive(Debug, Eq, PartialEq, Hash)]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            pub struct Folder {
                pub id: u32,
                pub name: String,
                pub owner_user_id: u32,
                pub parent: u32,
                pub passphrase: String,
                pub shared: bool,
                pub sort_by: String,
                pub sort_direction: String,
            }
        }
    }
}

pub mod setting {
    pub mod user {
        pub const API: &str = "SYNO.Foto.Setting.User";

        #[cfg(feature = "dto")]
        pub mod dto {
            #[cfg(feature = "serde")]
            use serde::{Deserialize, Serialize};

            #[derive(Debug, Eq, PartialEq, Hash)]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            pub struct UserSettings {
                pub enable_home_service: bool,
                pub enable_person: bool,
                pub team_space_permission: String,
                // ...
            }
        }
    }
}

pub mod user_info {
    pub const API: &str = "SYNO.Foto.UserInfo";

    #[cfg(feature = "dto")]
    pub mod dto {
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Eq, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
        pub struct UserInfo {
            pub id: u32,
            pub name: String,
            pub enabled: bool,
            pub is_migration_finished: bool,
            pub uid: u32,
            pub profile: UserProfile,
        }

        #[derive(Debug, Eq, PartialEq, Hash)]
        #[cfg_attr(
            feature = "serde",
            derive(Deserialize, Serialize),
            serde(rename_all = "camelCase")
        )]
        pub struct UserProfile {
            pub email: Option<String>,
            pub emails: Option<Vec<String>>,
            pub groups: Option<Vec<Group>>,
            pub id: Option<String>, // guid
            pub nick_name: Option<String>,
            pub photo: Option<String>,              // URL path and query
            pub preferred_color: Option<String>,    // hex RGB code, e.g. #1dbfbf
            pub preferred_language: Option<String>, // def for default
            pub timezone: Option<String>,
            pub timezone_u_i: Option<String>, // def for default
            pub title: Option<String>,        // ?
            pub uid: Option<u32>,
            pub user_name: String,
        }

        #[derive(Debug, Eq, PartialEq, Hash)]
        #[cfg_attr(
            feature = "serde",
            derive(Deserialize, Serialize),
            serde(rename_all = "camelCase")
        )]
        pub struct Group {
            pub gid: u32,
            pub id: String,
        }
    }
}

pub mod background_task {
    pub mod file {
        pub const API: &str = "SYNO.Foto.BackgroundTask.File";

        #[cfg(feature = "dto")]
        pub mod dto {
            #[cfg(feature = "serde")]
            use serde::{Deserialize, Serialize};

            #[derive(Debug, Eq, PartialEq, Hash)]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            pub struct TaskInfo {
                pub id: u32,
                pub operation: String,
                pub completion: u32,
                pub status: String,
                pub error: u32,
                pub overwrite: u32,
                pub skip: u32,
                pub create_time: u64,
                pub extra_info: String,
                pub target_folder: TargetFolder,
                pub total: u32,
            }

            #[derive(Debug, Eq, PartialEq, Hash)]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            pub struct TargetFolder {
                pub id: u32,
                pub owner_user_id: u32,
            }
        }
    }

    pub mod info {
        pub const API: &str = "SYNO.Foto.BackgroundTask.Info";

        #[cfg(feature = "dto")]
        pub mod dto {
            pub use super::super::file::dto::TaskInfo;
        }
    }
}

pub mod sharing {
    pub mod misc {
        pub const API: &str = "SYNO.Foto.Sharing.Misc";
    }
}

#[cfg(feature = "error")]
pub mod error {
    //! "Unofficial" errors detected so far.
    //!
    //! Unlike some of the other DSM APIs, Synology Photos API is not publicly documented, so these
    //! error codes are not guaranteed to always stay the same.
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Copy, Clone)]
    pub enum PhotoError {
        MissingPassphraseParameter = 609,
        NoAccessOrNotFound = 642,
    }

    impl Error for PhotoError {}

    impl Display for PhotoError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                PhotoError::MissingPassphraseParameter => {
                    write!(f, "missing passphrase for \"shared with me\" album")
                }
                PhotoError::NoAccessOrNotFound => {
                    write!(f, "no access or not found")
                }
            }
        }
    }

    impl TryFrom<u16> for PhotoError {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            let photo_err = match value {
                609 => PhotoError::MissingPassphraseParameter,
                642 => PhotoError::NoAccessOrNotFound,

                other => return Err(other),
            };
            Ok(photo_err)
        }
    }
}
