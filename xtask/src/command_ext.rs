use jod_thread::JoinHandle;
use std::path::PathBuf;

fn get_project_root_dir() -> PathBuf {
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
    fn run_watchable(
        &mut self,
        reason: &'static str,
        cond: bool,
        watchexec_args: &str,
    ) -> jod_thread::JoinHandle;
}

impl CommandExt for devx_cmd::Cmd {
    #[track_caller]
    fn run_it(&mut self, reason: &str) {
        self.run()
            .map_err(|err| {
                format!("Command for {reason:?} exited with non-zero code: {self:?}\n{err:#?}")
            })
            .unwrap();
    }
    #[track_caller]
    fn run_in_thread(&mut self, reason: &'static str) -> JoinHandle {
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
    fn run_watchable(
        &mut self,
        reason: &'static str,
        cond: bool,
        watchexec_args: &str,
    ) -> jod_thread::JoinHandle {
        if cond {
            let mut watchexec_cmd = devx_cmd::Cmd::new("watchexec");

            watchexec_cmd
                .args(watchexec_args.split(' '))
                .arg(self.get_bin())
                .args(self.get_args());

            if let Some(curr) = self.get_current_dir() {
                watchexec_cmd.current_dir(curr);
            }

            eprintln!("Running {watchexec_cmd}");

            watchexec_cmd.run_in_thread(reason)
        } else {
            self.run_in_thread(reason)
        }
    }
}
