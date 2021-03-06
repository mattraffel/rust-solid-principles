//! Logger module contains helper functions for using error!, debug!, trace! etc logging
use std::env;
use std::sync::{Once, ONCE_INIT};
use std::io::Write;

use env_logger::{Builder, fmt};
use log::{Record, Level, Metadata, Log, LevelFilter};

static LOGGER_INIT: Once = ONCE_INIT;

/**
    Routes logging to console all of the time regardless of RUST_LOG setting.  helpful for unit tests
*/
pub struct ConsoleLogger;

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("\r\n{:>5}|{:<30}|{:>35}:{:<4}| {}",
                record.level(),
                record.target(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            );
        }
    }

    fn flush(&self) {

    }
}

/**
    Required call to get logging to appear, depending on call (debug! vs error! etc)
    and RUST_LOG env setting.
*/
pub fn init_log() {
    LOGGER_INIT.call_once(|| {
        let format = |buf: &mut fmt::Formatter, record: &Record| {
            writeln!(
                buf,
                "{:>5}|{:<30}|{:>35}:{:<4}| {}",
                record.level(),
                record.target(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        };

        let mut builder = Builder::new();
        builder.format(format).filter(None, LevelFilter::Off);

        if env::var("RUST_LOG").is_ok() {
            builder.parse(&env::var("RUST_LOG").unwrap());
        }

        builder.init();

        trace!("logger initialized");
    });
}