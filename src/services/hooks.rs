use crate::utils::context::Context;
use anyhow::Result;

pub fn invalidate_caches(context: &Context, safe: &String) -> Result<()> {
    println!("{}", safe);
    Ok(())
}
