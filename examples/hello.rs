use anyhow::Error;

fn main() -> Result<(), Error> {
    rill::install()?;
    log::debug!("Hello, Rill!");
    Ok(())
}
