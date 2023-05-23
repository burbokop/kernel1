

use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct Logger;

impl Logger {
    pub fn init(&'static self) -> Result<(), SetLoggerError> {
        log::set_max_level(LevelFilter::Trace);
        log::set_logger(self)?;
        Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // TODO
        }
    }

    fn flush(&self) {}
}
