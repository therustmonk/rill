mod logger;

use log::LevelFilter;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("set logger error")]
    SetLoggerError(log::SetLoggerError),
}

pub fn install() -> Result<(), Error> {
    log::set_logger(&logger::RILL_LOGGER).map_err(Error::SetLoggerError)?;
    log::set_max_level(LevelFilter::Trace);
    Ok(())
}
