use std::time::{Duration, SystemTime};

/// Parameters from build time. Note that cargo won't regenerate this if source files don't change.
/// I recommend a post-commit hook to touch build.rs if you want revs marked clean.
#[derive(Clone, Debug)]
pub struct BuildStamp {
    /// Commit hash of `HEAD` at build time.
    pub git_revision: &'static str,

    /// Whether the working directory was clean at build time.
    pub git_revision_cleanness: &'static str,

    /// Git revision followed by a `*` if the status was not clean at the time.
    pub git_clean: bool,

    /// Output of git status at build time.
    pub git_status: &'static str,

    // Duration since Unix epoch.
    pub build_time_seconds: u64,
    pub build_time_nanos: u64,
}

impl BuildStamp {
    /// Commit hash of `HEAD` at build time.
    pub const fn git_revision(&self) -> &'static str {
        self.git_revision
    }

    /// Whether the working directory was clean at build time.
    pub const fn git_clean(&self) -> bool {
        self.git_clean
    }

    /// Git revision followed by a `*` if the status was not clean at the time.
    pub const fn git_revision_cleanness(&self) -> &'static str {
        self.git_revision_cleanness
    }

    /// Output of git status at build time.
    pub const fn git_status(&self) -> &'static str {
        self.git_status
    }

    /// Local system time of build.
    pub fn build_time(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH
            + Duration::from_secs(self.build_time_seconds)
            + Duration::from_nanos(self.build_time_nanos)
    }
}
