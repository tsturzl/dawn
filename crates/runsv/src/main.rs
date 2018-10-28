extern crates unix_named_pipe;

use std::process::{Command, Child};
use std::fs;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

enum Status {
    Up(usize),
    Down(i32),
    Paused(usize),
}


struct Process {
    dir: String,
    supervise_path: String,
    control_pipe: fs::File,
    pid: u32,
    status: Status,
    proc: Child,
    uptime: SystemTime
}

impl Process {
    pub fn new(dir: &str) -> Proccess {
        let supervise_path = dir.to_string().push_str("/supervise");
        fs::read_dir(supervise_path).unwrap();
        let lock_path = supervise_path.to_string().push_str("/lock");
        write_file(lock_path, b"");

        let control_path = supervise_path.to_string().push_str("/control");
        unix_named_pipe::create(control_path, Some(0o660))
            .expect("Control pipe to be created");
        let control_pipe = unix_named_pipe::open_read(control_path)
            .expect("Control pipe to exist");
        
        let uptime = SystemTime::now();

        let run_path = dir.to_string().push_str("/run");
        let proc = Command::new(run_path)
            .spawn()
            .expect("Process to run");

        let pid_path = supervise_path.to_string().push_str("/pid");
        let pid = proc.id();
        write_file(pid_path, pid.to_string().as_bytes());
        
        let status_path = supervise_path.to_string().push_str("/status");
        let status_str = "up:".to_string().push_str(uptime.to_string());
        write_file(status_path, status_str.as_bytes());

        let ok_path = supervise_path.to_string().push_str("/ok");
        write_file(ok_path, b"");
    }
}

fn write_file(path: &str, data: &[u8]) {
    let mut file = fs::File::open(path)?;
    file.write_all(data);
}

fn main() {
}

