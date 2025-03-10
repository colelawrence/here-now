use std::{
    borrow::BorrowMut,
    ffi::OsString,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{self, Stdio},
};
use sysinfo::SystemExt;

pub fn get_project_root_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR")
        .and_then(|value| PathBuf::from(value).parent().map(PathBuf::from))
        .expect("CARGO_MANIFEST_DIR was defined")
}

pub trait CommandExt {
    fn root_dir(&mut self, rel: &str) -> &mut Self;
    fn run_it(&mut self, reason: &str);
    fn run_in_thread(&mut self, reason: &'static str) -> CmdHandle;
    fn run_with_printer(&mut self, reason: &'static str, printer: for<'a> fn(&'a str))
        -> CmdHandle;
    fn arg_if(&mut self, cond: bool, arg: &str) -> &mut Self;
    fn args_if(&mut self, cond: bool, args_spaced: &str) -> &mut Self;
    fn env_if(&mut self, cond: bool, key: &str, value: &str) -> &mut Self;
    fn watchable(&mut self, cond: bool, watchexec_args: &str) -> &mut Self;
}

const ASCII_RED: &str = "\x1b[31m";
const ASCII_CYAN: &str = "\x1b[36m";
const ASCII_DIM: &str = "\x1b[2m";
const ASCII_RESET: &str = "\x1b[0m";

impl CommandExt for devx_cmd::Cmd {
    /// Will exit the process if the command fails.
    #[track_caller]
    #[tracing::instrument]
    fn run_it(&mut self, reason: &str) {
        tracing::span::Span::current().record("name", &reason);
        eprintln!("{ASCII_CYAN}{reason}\n$ {ASCII_DIM}{self:?}{ASCII_RESET}");
        match self.run() {
            Ok(_) => {}
            Err(err) => {
                tracing::error!(?err, ?reason, "Command exited with non-zero code");
                process::exit(1);
            }
        }
    }
    #[track_caller]
    #[tracing::instrument]
    fn run_in_thread(&mut self, reason: &'static str) -> CmdHandle {
        let current_span = tracing::span::Span::current();
        eprintln!("${ASCII_CYAN} {self:?}\n{ASCII_DIM}{reason}{ASCII_RESET}");
        let mut child = self
            .spawn()
            .map_err(|err| format!("Command for {reason:?} failed to start: {self:?}\n{err:#?}"))
            .unwrap();
        let self_debug = format!("{self:?}");
        let pid = sysinfo::Pid::from(child.child_mut().id() as usize);
        CmdHandle(
            pid,
            Some(jod_thread::spawn(move || {
                let _span = current_span.enter();
                match child.wait() {
                    Err(err) => {
                        tracing::error!(
                            reason,
                            self_debug,
                            "Command in thread exited with non-zero code: {err:#?}"
                        );
                    }
                    Ok(_) => {}
                }
            })),
        )
    }

    #[track_caller]
    fn run_with_printer(
        &mut self,
        reason: &'static str,
        printer: for<'a> fn(&'a str),
    ) -> CmdHandle {
        eprintln!("${ASCII_CYAN} {self:?}\n{ASCII_DIM}{reason}{ASCII_RESET}");
        let mut child = self
            .spawn_with(Stdio::inherit(), Stdio::piped())
            .map_err(|err| format!("Command for {reason:?} failed to start: {self:?}\n{err:#?}"))
            .unwrap();
        let self_debug = format!("{self:?}");
        let pid = sysinfo::Pid::from(child.child_mut().id() as usize);
        CmdHandle(
            pid,
            Some(jod_thread::spawn(move || {
                let buf = BufReader::new(child.child_mut().stderr.take().unwrap());
                for line in buf.lines() {
                    match line {
                        Ok(line) => printer(&line),
                        Err(err) => eprintln!("Line read error: {err:#?}"),
                    }
                }
                child.wait().map_err(|err| {
                    format!("{ASCII_RED} Command for {reason:?} in thread exited with non-zero code: {self_debug:?}\n{ASCII_DIM}{err:#?}{ASCII_RESET}")
                }).unwrap()
            })),
        )
    }

    fn arg_if(&mut self, cond: bool, arg: &str) -> &mut Self {
        if cond {
            self.arg(arg)
        } else {
            self
        }
    }

    fn args_if(&mut self, cond: bool, args_spaced: &str) -> &mut Self {
        if cond {
            self.args(args_spaced.split(' '))
        } else {
            self
        }
    }

    fn env_if(&mut self, cond: bool, key: &str, value: &str) -> &mut Self {
        if cond {
            self.env(key, value)
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

    fn watchable(&mut self, cond: bool, watchexec_args: &str) -> &mut Self {
        if cond {
            let bin = self.get_bin();
            let args = self.get_args().to_vec();
            let arg_len = args.len();
            let mut all_args_it = watchexec_args
                .split(' ')
                .chain("--on-busy-update restart".split(' '))
                .map(OsString::from)
                .chain(std::iter::once(OsString::from(bin)))
                .chain(args)
                .into_iter();

            self.bin("watchexec");
            for (idx, arg) in all_args_it.borrow_mut().take(arg_len).enumerate() {
                self.replace_arg(idx, arg);
            }

            self.args(all_args_it);
        }
        self
    }
}

/// Kills the underlying command on drop.
pub struct CmdHandle(sysinfo::Pid, Option<jod_thread::JoinHandle>);

impl CmdHandle {
    pub fn join(mut self) {
        self.join_mut();
    }
    fn join_mut(&mut self) {
        if let Some(handle) = self.1.take() {
            handle.join()
        }
    }
    pub fn kill(mut self) {
        self.kill_mut()
    }
    fn kill_mut(&mut self) {
        let sys = sysinfo::System::new_all();
        if let Some(proc) = sys.process(self.0) {
            sysinfo::ProcessExt::kill(proc);
        }
        self.join_mut();
    }
}

impl Drop for CmdHandle {
    fn drop(&mut self) {
        self.kill_mut();
    }
}
