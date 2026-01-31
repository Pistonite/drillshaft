use std::path::{Path, PathBuf};

fn main() -> cu::Result<()> {
    let config_path = env!("SHAFT_SHIM_BUILD_CONFIG");
    let main_rs = shaftim_build::build(Path::new(config_path))?;

    let mut main_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    main_path.push("main.rs");
    if let Err(e) = cu::fs::write(main_path, main_rs) {
        eprintln!("cargo::error={e}");
    }
    Ok(())
}
