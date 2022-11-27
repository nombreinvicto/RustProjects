// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let message = match level {
        LogLevel::Info => format!("[INFO]: {message}", message = message),
        LogLevel::Error => format!("[ERROR]: {message}", message = message),
        LogLevel::Warning => format!("[WARNING]: {message}", message = message),
        LogLevel::Debug => format!("[DEBUG]: {message}", message = message),
    };

    return message;
}

pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}

pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}

pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
