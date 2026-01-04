use std::io::Read;
use std::path::Path;

use sha2::{Sha256, Digest};

/// Get the SHA256 checksum of a file and return it as a string
#[cu::error_ctx("failed to hash file: '{}'", path.display())]
pub fn file_sha256(path: &Path) -> cu::Result<String> {
    let mut hasher = Sha256::new();
    let mut reader = cu::fs::reader(&path)?;
    let mut buf = vec![0u8;409600].into_boxed_slice();
    loop {
        let i = reader.read(&mut buf)?;
        if i == 0 {
            break;
        }
        hasher.update(&buf[..i]);
    }
    let result = hasher.finalize();
    let mut out = String::with_capacity(64);
    let digits = b"0123456789abcdef";
    for b in result {
        let c1 = digits[(b / 16) as usize] as char;
        let c2 = digits[(b % 16) as usize] as char;
        out.push(c1);
        out.push(c2);
    }
    Ok(out)
}
