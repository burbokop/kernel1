
use core::cell::RefCell;

use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

use crate::hw::serial::{Port, InitedPort};

pub struct Logger {
    port: InitedPort
}

impl Logger {
    pub fn new(port: Port) -> Option<Logger> {
        match unsafe { port.init() } {
            Some(port) => Some(Logger { port }),
            None => None,
        }
    }

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
            let port = unsafe { &mut *(&self.port as *const InitedPort as *mut InitedPort) };

            use core::fmt::Write;
            writeln!(port, "{:<5} {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}
