use anyhow::Result;
use tinykv::init;

fn main() -> Result<()> {
    init()?;
    Ok(())
}
