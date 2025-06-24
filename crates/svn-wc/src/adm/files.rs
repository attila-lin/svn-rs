use super::Adm;
use super::AdmError;
use super::DEFAULT_ADM_DIR_NAME;

impl Adm {
    /// `svn_wc_set_adm_dir`
    pub fn set_adm_dir(name: &str) -> Result<(), AdmError> {
        const VALID_DIR_NAMES: [&str; 2] = [DEFAULT_ADM_DIR_NAME, "_svn"];

        if !VALID_DIR_NAMES.contains(&name) {
            Ok(())
        } else {
            Err(AdmError::InvalidAdmDirName(name.to_string()))
        }
    }
}
