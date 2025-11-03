use std::path::Path;

fn main() -> cu::Result<()> {
    let crate_path = std::env::var("CARGO_MANIFEST_DIR")?;
    drillshaft_build::build_registry(Path::new(&crate_path))
}
