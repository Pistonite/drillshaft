use crate::pre::*;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SddmConfig {
    autologin: SddmAutoLoginConfig
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SddmAutoLoginConfig {
    /// Username for SDDM auto-login
    user: String,
}
pub fn configure(cfg: &SddmConfig) -> cu::Result<()> {
    opfs::sudo("systemctl", "enabling sddm service")?
        .args(["enable", "sddm.service"])
        .stdoe(cu::lv::P)
        .stdin_null()
        .wait_nz()?;
    let current_config = Path::new("/etc/sddm.conf.d/default.conf");
    if !current_config.exists() {
        let default_config = Path::new("/usr/lib/sddm/sddm.conf.d/default.conf");
        cu::fs::copy(default_config, current_config)?;
    }
    let mut ini = opfs::IniFile::open(current_config)?;
    let section = ini.section_mut("Autologin");
    section.set("Relogin", "true");
    section.set("Session", "wayland");
    section.set("User", &cfg.autologin.user);
    ini.write()?;


    Ok(())
}
