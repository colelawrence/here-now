use std::process::{exit, Command};

use jod_thread::JoinHandle;

use crate::get_project_root_dir;

pub trait CommandExt {
    fn root_dir(&mut self, rel: &str) -> &mut Self;
    fn run(&mut self, reason: &str);
    fn run_in_thread(&mut self, reason: &'static str) -> JoinHandle;
    fn arg_if(&mut self, cond: bool, arg: &str) -> &mut Self;
}

impl CommandExt for Command {
    #[track_caller]
    fn run(&mut self, reason: &str) {
        let status = self.status().expect("ran command");
        if !status.success() {
            eprintln!("Command for {reason:?} exited with non-zero code: {self:?}");
            std::process::exit(status.code().unwrap_or(1))
        }
    }
    #[track_caller]
    fn run_in_thread(&mut self, reason: &'static str) -> JoinHandle {
        let mut child = self.spawn().expect("spawned child");
        let self_debug = format!("{self:?}");
        jod_thread::spawn(move || {
            let status = child.wait().expect("exiting");
            if !status.success() {
                eprintln!(
                    "Command for {reason:?} in thread exited with non-zero code: {self_debug}"
                );
                exit(status.code().unwrap_or(1))
            }
        })
    }

    fn arg_if(&mut self, cond: bool, arg: &str) -> &mut Self {
        if cond {
            self.arg(arg)
        } else {
            self
        }
    }

    #[track_caller]
    fn root_dir(&mut self, rel: &str) -> &mut Self {
        self.current_dir(
            get_project_root_dir()
                .join(rel)
                .canonicalize()
                .expect("found directory"),
        )
    }
}
