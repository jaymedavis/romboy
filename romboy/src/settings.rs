use config::{Config, ConfigError};
use directories::UserDirs;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Settings {
    path: Path,
    pub(crate) platforms: HashMap<String, Vec<String>>,
    pub(crate) logging: Logging,
}

#[derive(Debug, Deserialize, Clone)]
struct Path {
    zips: String,
    roms: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Logging {
    pub(crate) mode: String,
}

impl Settings {
    pub(crate) fn new() ->  Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("settings"))
            .build()?;

        settings.try_deserialize()
    }

    pub(crate) fn zips_path(&self) -> String {
        Self::replace_tilde_with_home_dir(&self.path.zips)
    }

    pub(crate) fn roms_path(&self) -> String {
        Self::replace_tilde_with_home_dir(&self.path.roms)
    }

    fn replace_tilde_with_home_dir(path: &str) -> String {
        let user_dirs = UserDirs::new().unwrap();
        let home_dir = user_dirs.home_dir().to_str().unwrap();

        path.replace("~", home_dir)
    }
}
