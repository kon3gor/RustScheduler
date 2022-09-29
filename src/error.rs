use std::{fmt::Display, io, str::Utf8Error};
use toml::de;
use daemonize::DaemonizeError;

#[derive(Debug)]
pub struct SchedulerError {
    msg: String,
}

impl From<io::Error> for SchedulerError {
    fn from(e: io::Error) -> Self {
        let msg = format!("io error while reading task.toml: {}", e);
        return SchedulerError { msg };
    }
}

impl From<Utf8Error> for SchedulerError {
    fn from(e: Utf8Error) -> Self {
        let msg = format!("Parsing error while reading task.toml: {}", e);
        return SchedulerError { msg };
    }
}

impl From<de::Error> for SchedulerError {
    fn from(e: de::Error) -> Self {
        let msg = format!("Parsing error while reading task.toml: {}", e);
        return SchedulerError { msg };
    }
}

impl Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<DaemonizeError> for SchedulerError {
    fn from(e: DaemonizeError) -> Self {
        let msg = format!("Error while starting daemon: {}", e);
        return SchedulerError { msg };
    }
}
