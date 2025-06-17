use std::path::Path;

/// Opaque structure describing a set of configuration options.
///
/// `svn_config_t`
#[derive(Debug)]
pub struct SvnConfig {}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {}

impl SvnConfig {
    pub fn from_path(path: &Path) -> Self {
        todo!()
    }

    /// load the configuration from the system registry path
    #[cfg(windows)]
    fn load_from_sys_registry_path(sys_registry_path: &str) -> Result<SvnConfig, ConfigError> {
        todo!()
    }

    #[cfg(not(windows))]
    fn load_from_user_registry_path(user_registry_path: &str) -> Result<SvnConfig, ConfigError> {
        todo!()
    }

    /// load the configuration from the system config file
    pub fn load_from_sys_config_file(sys_config_file: &str) -> Result<SvnConfig, ConfigError> {
        todo!()
    }

    /// load the configuration from the user config file
    pub fn load_from_user_config_file(user_config_file: &str) -> Result<SvnConfig, ConfigError> {
        todo!()
    }

    /// `svn_config_get_bool`
    pub fn get_bool(
        &self,
        section: &str,
        option: &str,
        default_value: bool,
    ) -> Result<bool, ConfigError> {
        todo!()
    }

    /// `svn_config_get_int64`
    pub fn get_i64(
        &self,
        section: &str,
        option: &str,
        default_value: i64,
    ) -> Result<i64, ConfigError> {
        todo!()
    }
}
