use std::collections::BTreeSet;

use cu::pre::*;

pub(crate) struct Pacman {
    installed_packages: BTreeSet<String>,
}

crate::main_thread! {
    const fn pacman() -> Pacman {
        Pacman {
            installed_packages: BTreeSet::new(),
        }
    }
}

/// Check if a package is installed with pacman
pub fn is_installed(package_name: &str) -> cu::Result<bool> {
    let mut state = pacman::instance()?;
    let not_loaded = state.installed_packages.is_empty();
    
    if not_loaded {
        cu::debug!("pacman: querying installed packages");
        let (child, stdout) = cu::which("pacman")?
            .command()
            .arg("-Qq")
            .stdout(cu::pio::string())
            .stdie_null()
            .spawn()?;
        child.wait_nz()?;
        let stdout = stdout.join()??;
        state.installed_packages .extend(stdout.lines().map(|x| x.trim().to_string()
        ));
    }
    Ok(state.installed_packages.contains(package_name))
}
