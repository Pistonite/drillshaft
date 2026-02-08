use cu::pre::*;

use crate::{hmgr, opfs};

static TOOLS_TAR_GZ: &[u8] = include_bytes!("./tools.tar.gz");
#[path = "tools_targz.gen.rs"]
#[rustfmt::skip]
mod _gen;

/// Ensure the tools directory is unpacked and up to date
pub fn ensure_unpacked() -> cu::Result<()> {
    let need_unpack = !matches!(_gen::TOOLS_VERSION.is_uptodate(), Ok(Some(true)));
    if need_unpack {
        cu::check!(do_unpack(), "failed to unpack tools")?;
    }
    Ok(())
}

fn do_unpack() -> cu::Result<()> {
    cu::info!("unpacking tools...");
    let tools_path = hmgr::paths::tools_root();
    cu::check!(
        opfs::untargz_read(TOOLS_TAR_GZ, &tools_path, true /* clean */),
        "failed to unpack tools"
    )?;
    _gen::TOOLS_VERSION.update()?;
    Ok(())
}
