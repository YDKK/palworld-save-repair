use anyhow::Result;
use vergen::EmitBuilder;

pub fn main() -> Result<()> {
    EmitBuilder::builder()
        .git_commit_date()
        .git_sha(true)
        .emit()?;
    Ok(())
}
