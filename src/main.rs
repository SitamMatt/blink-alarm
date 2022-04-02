#![windows_subsystem = "windows"]

use job_scheduler::{Job, JobScheduler};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::result::Result;
use std::time::Duration;
use std::env;
use winrt_notification::{Sound, Toast};

const DEFAULT_CONFIGURATION: &str = include_str!("../configuration_example.json");

#[derive(Deserialize)]
struct Configuration {
    schedule: String,
}

fn read_configuration_from_file(path: &str) -> Result<Configuration, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config: Configuration = serde_json::from_reader(reader)?;

    Ok(config)
}

fn read_configuration_from_string(str: &str) -> Result<Configuration, Box<dyn Error>> {
    let config: Configuration = serde_json::from_str(str)?;

    Ok(config)
}

#[cfg(windows)]
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = if args.len() > 1 {
        read_configuration_from_file(&args[1]).unwrap()
    } else {
        read_configuration_from_string(DEFAULT_CONFIGURATION).unwrap()
    };

    let mut sched = JobScheduler::new();

    sched.add(Job::new(config.schedule.parse().unwrap(), || {
        Toast::new(Toast::POWERSHELL_APP_ID)
            .title("Blink!")
            .text1("Blink your eyes")
            .sound(Some(Sound::Default))
            .duration(winrt_notification::Duration::Short)
            .show()
            .expect("unable to toast");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_secs(1));
    }
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows");
}
