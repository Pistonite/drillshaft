use std::{collections::BTreeSet, sync::Mutex};

use cu::pre::*;

static STATE: Mutex<State> = Mutex::new(State::new());

struct State {
    installed_packages: BTreeSet<String>,
}
impl State {
    pub const fn new() -> Self {
        Self {
            installed_packages: BTreeSet::new(),
        }
    }
}

pub fn is_installed(package_name: &str) -> cu::Result<bool> {
    let mut state = STATE.lock()?;
    if state.installed_packages.is_empty() {
        let (child, stdout) = cu::which("pacman")?
            .command()
            .arg("-Qq")
            .stdout(cu::pio::string())
            .stdie_null()
            .spawn()?;
        child.wait_nz()?;
        let stdout = stdout.join()??;
        state
            .installed_packages
            .extend(stdout.lines().map(|x| x.trim().to_string()))
    }
}
