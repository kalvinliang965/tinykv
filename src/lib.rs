// lib.rs
pub mod config;
pub mod error;
pub mod kv;
pub mod raft;
pub mod storage;
pub mod coordinator;
pub mod util;

use anyhow::Result;


pub fn init() -> Result<()> {
    util::logging::init_logging();
    tracing::info!("tinykv starting up");
    Ok(())
}
