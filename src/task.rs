use serde_derive::Deserialize;
use std::{fs, path::PathBuf};
use toml;
use crate::error::SchedulerError;

#[derive(Deserialize)]
pub struct Task {
    pub cmd: String,
    pub day: String,
    pub time: String,
}

pub fn load_task(cwd: &PathBuf) -> Result<Task, SchedulerError> {
    let cwd = cwd.join("task.toml");
    let task_file = fs::read(cwd)?;
    let task_str = std::str::from_utf8(&task_file)?;
    let task = toml::from_str(task_str)?;
    return Ok(task);
}
