//! SYNO.FotoTeam (Synology Photos, sharing features)
//!
//! Note that Synology Photos API is not public, so official documentation is not available.
//! All the contents of this module have been reverse-engineered from Synology Photos web app.

pub mod browse {
    pub mod folder {
        #[cfg(feature = "dto")]
        pub use crate::foto::browse::folder::dto::Folder;

        pub const API: &str = "SYNO.FotoTeam.Browse.Folder";
    }

    pub mod person {
        #[cfg(feature = "dto")]
        pub use crate::foto::browse::person::dto::Person;

        pub const API: &str = "SYNO.FotoTeam.Browse.Person";
    }

    pub mod item {
        #[cfg(feature = "dto")]
        pub use crate::foto::browse::item::dto::Item;

        pub const API: &str = "SYNO.FotoTeam.Browse.Item";
    }
}

pub mod background_task {
    pub mod file {
        pub const API: &str = "SYNO.FotoTeam.BackgroundTask.File";

        #[cfg(feature = "dto")]
        pub use crate::foto::background_task::file::dto::TaskInfo;
    }
}

#[cfg(feature = "error")]
pub use super::foto::error;
