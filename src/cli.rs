use std::{path::PathBuf, io::Write};
use clap::Parser;
use std::{fs, env};
use crate::error::SchedulerError;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    name: String,

    #[arg(short, long)]
    command: String,

    #[arg(short, long)]
    day: String,

    #[arg(short, long)]
    time: String,
}

impl Args {
    pub fn process(&self) -> Result<PathBuf, SchedulerError> {
        let path = Self::create_folder(&self)?;
        Self::create_task_file(&self, &path)?;
        return Ok(path);
    }

    fn create_folder(&self) -> Result<PathBuf, SchedulerError> {
        let cwd = env::current_dir()?.join(&self.name);
        fs::create_dir_all(&cwd)?;
        return Ok(cwd);
    }

    fn create_task_file(&self, path: &PathBuf) -> Result<(), SchedulerError> {
        let cwd = path.join("task.toml");
        let mut file = fs::File::create(cwd)?;
        let formatted_body = format!(
            "cmd=\"{}\"\nday=\"{}\"\ntime=\"{}\"", 
            self.command, 
            self.day, 
            self.time
        );
        file.write_all(formatted_body.as_bytes())?;
        return Ok(());
    }
}
