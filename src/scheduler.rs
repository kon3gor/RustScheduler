use chrono::{FixedOffset, TimeZone, Utc};
use daemonize::Daemonize;
use sh_inline::*;
use std::fs::File;
use std::thread;
use std::time::Duration;
use crate::task::*;
use crate::time_utils::{check_time, check_day};
use std::path::PathBuf;
use crate::error::SchedulerError;

type Result<ErrType> = std::result::Result<(), ErrType>;

pub fn run_scheduler(cwd: &PathBuf) -> Result<SchedulerError> {
    let stdout = File::create(cwd.join("scheduler.out"))?;
    let stderr = File::create(cwd.join("scheduler.err"))?;

    let daemonize = Daemonize::new()
        .pid_file(cwd.join("scheduler.pid")) // Every method except `new` and `start`
        .working_directory(".") // for default behaviour.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    daemonize.start()?;
    main_loop(&cwd)?;

    return Ok(());
}


fn main_loop(cwd: &PathBuf) -> Result<SchedulerError> {
    let task = load_task(&cwd)?;
    loop {
        let should_run_task = check_task_criteria(&task);
        if should_run_task {
            match bash!(task.cmd.as_str()) {
                Ok(v) => v,
                Err(e) => eprintln!("Error while executing command: {}", e),
            };
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn check_task_criteria(task: &Task) -> bool {
    let now_utc = Utc::now().naive_utc();
    let current_time = FixedOffset::east(60 * 60 * 3).from_utc_datetime(&now_utc);
    let does_day_suitable = check_day(&task.day, &current_time);
    let does_time_suitable = check_time(&task.time, &current_time);
    return does_day_suitable && does_time_suitable;
}


