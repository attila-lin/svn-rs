/// Opaque structure describing a set of configuration options.
///
/// `svn_config_t`
pub struct SvnConfig {}

impl SvnConfig {
    /// load the configuration from the system registry path
    #[cfg(target_os = "windows")]
    fn load_from_sys_registry_path(sys_registry_path: &str) -> Result<SvnConfig, String> {
        todo!()
    }

    #[cfg(target_os = "windows")]
    fn load_from_user_registry_path(user_registry_path: &str) -> Result<SvnConfig, String> {
        todo!()
    }

    /// load the configuration from the system config file
    pub fn load_from_sys_config_file(sys_config_file: &str) -> Result<SvnConfig, String> {
        todo!()
    }

    /// load the configuration from the user config file
    pub fn load_from_user_config_file(user_config_file: &str) -> Result<SvnConfig, String> {
        todo!()
    }
}
