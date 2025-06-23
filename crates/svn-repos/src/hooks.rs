//! hooks.c : running repository hooks

use std::collections::HashMap;
use std::process::Command;

/// From `run_hook_cmd`
pub struct Hook {
    name: String,
    cmd: String,
    args: Vec<String>,
    hooks_env: HashMap<String, String>,
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum HookError {}

impl Hook {
    /// Helper function for run_hook_cmd().  Wait for a hook to finish
    /// executing and return either SVN_NO_ERROR if the hook script completed
    /// without error, or an error describing the reason for failure.
    ///
    /// NAME and CMD are the name and path of the hook program, CMD_PROC
    /// is a pointer to the structure representing the running process,
    /// and READ_ERRHANDLE is an open handle to the hook's stderr.
    ///
    /// Hooks are considered to have failed if we are unable to wait for the
    /// process, if we are unable to read from the hook's stderr, if the
    /// process has failed to exit cleanly (due to a coredump, for example),
    /// or if the process returned a non-zero return code.
    ///
    /// Any error output returned by the hook's stderr will be included in an
    /// error message, though the presence of output on stderr is not itself
    /// a reason to fail a hook.
    ///
    /// `check_hook_result`
    pub fn check_result(&self) -> Result<(), HookError> {
        todo!()
    }
}
