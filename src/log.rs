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

#[macro_export]
macro_rules! log {
    ($level:path, $($arg:tt)*) => {
        {
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
            eprint!($($arg)*);
        }
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
