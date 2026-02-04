/// Macros for generating the package definition
mod generator;
pub(crate) use generator::*;
/// Macros for implementing verify() of the package
mod verify;
pub(crate) use verify::*;

macro_rules! test_config {
    ($IDENT:ident) => {
        #[cfg(test)]
        mod test_config {
            #[test]
            fn parse_default_config() -> cu::Result<()> {
                super::$IDENT.load_default()?;
                Ok(())
            }
        }
    };
}
pub(crate) use test_config;

#[cfg(target_os = "linux")]
macro_rules! check_installed_pacman_package {
    ($l:literal) => {
        match corelib::epkg::pacman::installed_version($l)? {
            None => {
                return Ok(Verified::NotInstalled);
            }
            Some(x) => x,
        }
    };
}
#[cfg(target_os = "linux")]
pub(crate) use check_installed_pacman_package;

#[cfg(target_os = "linux")]
macro_rules! check_installed_with_pacman {
    ($bin:literal, $l:literal) => {
        check_bin_in_path!($bin);
        match corelib::epkg::pacman::installed_version($l)? {
            None => {
                cu::bail!("current '{}' is not installed with pacman; please uninstall it", $bin)
            }
            Some(x) => x,
        }
    };
    ($bin:literal, $l:literal, $system:literal) => {
        check_bin_in_path!($bin);
        match corelib::epkg::pacman::installed_version($l)? {
            None => {
                cu::bail!("current '{}' is not installed with pacman; please uninstall it or use the '{}' package", $bin, $system)
            }
            Some(x) => x,
        }
    };
}
#[cfg(target_os = "linux")]
pub(crate) use check_installed_with_pacman;
