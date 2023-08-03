use jod_thread::JoinHandle;
use std::{borrow::BorrowMut, ffi::OsString, path::PathBuf};

pub fn get_project_root_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR")
        .and_then(|value| PathBuf::from(value).parent().map(PathBuf::from))
        .expect("CARGO_MANIFEST_DIR was defined")
}

pub trait CommandExt {
    fn root_dir(&mut self, rel: &str) -> &mut Self;
    fn run_it(&mut self, reason: &str);
    fn run_in_thread(&mut self, reason: &'static str) -> JoinHandle;
    fn arg_if(&mut self, cond: bool, arg: &str) -> &mut Self;
    fn env_if(&mut self, cond: bool, key: &str, value: &str) -> &mut Self;
    fn watchable(&mut self, cond: bool, watchexec_args: &str) -> &mut Self;
}
const ASCII_CYAN: &str = "\x1b[36m";
const ASCII_DIM: &str = "\x1b[2m";
const ASCII_RESET: &str = "\x1b[0m";

impl CommandExt for devx_cmd::Cmd {
    #[track_caller]
    fn run_it(&mut self, reason: &str) {
        eprintln!("${ASCII_CYAN} {self:?}\n{ASCII_DIM}{reason}{ASCII_RESET}");
        self.run()
            .map_err(|err| {
                format!("Command for {reason:?} exited with non-zero code: {self:?}\n{err:#?}")
            })
            .unwrap();
    }
    #[track_caller]
    fn run_in_thread(&mut self, reason: &'static str) -> JoinHandle {
        eprintln!("${ASCII_CYAN} {self:?}\n{ASCII_DIM}{reason}{ASCII_RESET}");
        let mut child = self
            .spawn()
            .map_err(|err| format!("Command for {reason:?} failed to start: {self:?}\n{err:#?}"))
            .unwrap();
        let self_debug = format!("{self:?}");
        jod_thread::spawn(move || {
            child.wait().map_err(|err| {
                format!("Command for {reason:?} in thread exited with non-zero code: {self_debug:?}\n{err:#?}")
            }).unwrap()
        })
    }

    fn arg_if(&mut self, cond: bool, arg: &str) -> &mut Self {
        if cond {
            self.arg(arg)
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
