use log::{Level, Log, Metadata, Record};

pub static RILL_LOGGER: RillLogger = RillLogger;

pub struct RillLogger;

impl Log for RillLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
