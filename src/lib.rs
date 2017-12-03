#![deny(warnings)]
#![deny(missing_docs)]

//! A logger configured via an environment variable which writes to standard
//! error with nice colored output for log levels.
//!
//! ## Example
//!
//! ```
//! extern crate pretty_env_logger;
//! #[macro_use] extern crate log;
//!
//! fn main() {
//!     pretty_env_logger::init().unwrap();
//!
//!     trace!("a trace example");
//!     debug!("deboogging");
//!     info!("such information");
//!     warn!("o_O");
//!     error!("boom");
//! }
//! ```

extern crate ansi_term;
extern crate env_logger;
extern crate log;

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use ansi_term::{Color, Style};
use env_logger::LogBuilder;
use log::LogLevel;

struct Level(LogLevel);

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            LogLevel::Trace => Color::Purple.paint("TRACE"),
            LogLevel::Debug => Color::Blue.paint("DEBUG"),
            LogLevel::Info => Color::Green.paint("INFO "),
            LogLevel::Warn => Color::Yellow.paint("WARN "),
            LogLevel::Error => Color::Red.paint("ERROR")
        }.fmt(f)
    }
}

static MAX_MODULE_WIDTH: AtomicUsize = ATOMIC_USIZE_INIT;

/// Initializes the global logger with a pretty env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
#[inline]
pub fn init() -> Result<(), log::SetLoggerError> {
    init_custom_env("RUST_LOG")
}

/// Initialized the global logger with a pretty env logger, with a custom variable name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn init_custom_env(environment_variable_name: &str) -> Result<(), log::SetLoggerError> {
    let mut builder = LogBuilder::new();

    builder.format(|record| {
        let mut module_path = record.location().module_path().to_string();
        let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
        if max_width > module_path.len() {
            let diff = max_width - module_path.len();
            module_path.extend(::std::iter::repeat(' ').take(diff));
        } else {
            MAX_MODULE_WIDTH.store(module_path.len(), Ordering::Relaxed);
        }
        format!("{}:{}: {}",
                Level(record.level()),
                Style::new().bold().paint(module_path),
                record.args())
    });

    if let Ok(s) = ::std::env::var(environment_variable_name) {
        builder.parse(&s);
    }

    builder.init()
}
