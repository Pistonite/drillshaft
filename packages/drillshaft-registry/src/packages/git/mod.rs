
//! Git version control System

use cu::pre::*;

use crate::pre::*;

metadata_binaries!("git");

pub fn verify(ctx: &Context) -> cu::Result<Verified> {
    if cu::which("git").is_err() {
        return Ok(Verified::NotInstalled);
    }

}

pub mod version;
