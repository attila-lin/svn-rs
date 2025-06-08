use std::rc::Rc;

/// The Repository object, created by svn_repos_open2() and
//    svn_repos_create().
pub struct Repos {
    // A Subversion filesystem object.
    // fs: Rc<Fs>
}

impl Repos {
    /// Opens a repository at the given path.
    pub fn open(repository_path: &str) -> Self {
        // Here we would normally open the repository and return a Repos instance.
        // For now, we just return an empty Repos instance.
        Repos {
            // fs: Rc::new(Fs::new(repository_path)),
        }
    }

    /// Creates a new repository at the given path.
    pub fn create(repository_path: &str) -> Self {
        // Here we would normally create a new repository and return a Repos instance.
        // For now, we just return an empty Repos instance.
        Repos {
            // fs: Rc::new(Fs::new(repository_path)),
        }
    }
}
