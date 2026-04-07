//! Logging infra

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl Level {
    #[cfg(target_os = "horizon")]
    fn get_horizon_level(self) -> nx::diag::log::LogSeverity {
        match self {
            Level::Trace => nx::diag::log::LogSeverity::Trace,
            Level::Debug => nx::diag::log::LogSeverity::Trace,
            Level::Info => nx::diag::log::LogSeverity::Info,
            Level::Warning => nx::diag::log::LogSeverity::Warn,
            Level::Error => nx::diag::log::LogSeverity::Error,
            Level::Fatal => nx::diag::log::LogSeverity::Fatal,
        }
    }
    #[cfg(target_os = "horizon")]
    fn get_horizon_verbose(self) -> bool {
        match self {
            Level::Trace => true,
            Level::Debug => true,
            Level::Info => true,
            Level::Warning => false,
            Level::Error => false,
            Level::Fatal => false,
        }
    }
}

#[macro_export]
#[cfg(not(target_os = "horizon"))]
macro_rules! log {
    ($level:path: $verbose:literal, $($arg:tt)*) => {
        use $crate::log::Level::*;
        eprint!("{}", match $level {
            Trace => "[TRACE] ",
            Debug => "\x1b[32m[DEBUG] ",
            Info => "\x1b[34m[INFO ] ",
            Warning => "\x1b[33m[WARN ] ",
            Error => "\x1b[31m[ERROR] ",
            Fatal => "\x1b[31;1m[FATAL] ",
        });
        eprint!("\x1b[0m");
        eprintln!($($arg)*);
    };
    ($level:path, $($arg:tt)*) => {
        eprint!("{}", match $level {
            $crate::log::Level::Trace => "[TRACE] ",
            $crate::log::Level::Debug => "\x1b[32m[DEBUG] ",
            $crate::log::Level::Info => "\x1b[34m[INFO ] ",
            $crate::log::Level::Warning => "\x1b[33m[WARN ] ",
            $crate::log::Level::Error => "\x1b[31m[ERROR] ",
            $crate::log::Level::Fatal => "\x1b[31;1m[FATAL] ",
        });
        eprint!("\x1b[0m");
        eprintln!($($arg)*);
    };
}

#[macro_export]
#[cfg(target_os = "horizon")]
macro_rules! log {
    ($level:path: $verbose:literal, $($arg:tt)*) => {
        $crate::diag_log!(log::lm::LmLogger { ($level).get_horizon_level(), $verbose } => $($arg)*);
    };
    ($level:path, $($arg:tt)*) => {
        $crate::diag_log!(log::lm::LmLogger { ($level).get_horizon_level(), ($level).get_horizon_verbose() } => $($arg)*);
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Trace, $($arg)*);
    };
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Debug, $($arg)*);
    };
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Info, $($arg)*);
    };
}
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Warning, $($arg)*);
    };
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Warning, $($arg)*);
    };
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Error, $($arg)*);
    };
}
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Error, $($arg)*);
    };
}
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Fatal, $($arg)*);
    };
}