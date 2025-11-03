use enumset::{EnumSet, EnumSetType};

/// Platform supported by the package manager
#[derive(EnumSetType)]
#[repr(u8)]
pub enum Platform {
    /// Unknown or unsupported platform
    Unknown,
    /// Windows
    Windows,
    /// Arch Linux
    Arch,
}
impl From<u8> for Platform {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Windows,
            2 => Self::Arch,
            _ => Self::Unknown,
        }
    }
}
impl From<Platform> for u8 {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Unknown => 0,
            Platform::Windows => 1,
            Platform::Arch => 2,
        }
    }
}

impl Platform {
    pub const fn all() -> EnumSet<Self> {
        enumset::enum_set! {
            Self::Windows |
            Self::Arch
        }
    }
}

static CURRENT_PLATFORM: cu::Atomic<u8, Platform> = cu::Atomic::new_u8(0);

/// Initialize the platform variable. Called once at beginning when launching
/// the package manager
#[inline(always)]
pub fn init_platform() -> cu::Result<()> {
    init_platform_impl()
}

#[cfg(windows)]
#[inline(always)]
fn init_platform_impl() -> cu::Result<()> {
    CURRENT_PLATFORM.set(Platform::Windows);
    Ok(())
}

#[cfg(not(windows))]
#[inline(always)]
fn init_platform_impl() -> cu::Result<()> {
    use std::path::Path;

    if Path::new("/etc/arch-release").exists() {
        if cu::which("pacman").is_err() {
            cu::bail!("unsupported platform: pacman not available; please fix your system");
        }
        CURRENT_PLATFORM.set(Platform::Arch);
        return Ok(());
    }

    cu::bail!("cannot determine the platform of the system");
}
