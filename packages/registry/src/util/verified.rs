/// Package verification status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verified {
    /// Everything is up-to-date
    UpToDate,
    /// Installed but not up-to-date
    NotUpToDate,
    /// Not installed
    NotInstalled,
    /// Installation is up-to-date, but needs to be re-configured
    NeedsConfig,
}
impl Verified {
    pub const fn is_uptodate(uptodate: bool) -> Self {
        if uptodate {
            Self::UpToDate
        } else {
            Self::NotUpToDate
        }
    }
}
