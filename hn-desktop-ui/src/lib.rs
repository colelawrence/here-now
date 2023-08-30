use std::{cell::RefCell, collections::HashMap, rc::Rc};

use hn_hinted_id::HintedID;
use slint::ComponentHandle;
use tracing::*;

mod slint_ui {
    pub use ui_main_window::*;
    pub use ui_settings_profile_server_window::*;
    pub use ui_settings_profile_window::*;
}

struct MainUI {
    // windows_shown: Arc<atomic::AtomicU8>,
    main_window: Option<slint_ui::HereNowMainWindow>,
    profile_settings_windows: HashMap<HintedID, slint_ui::HereNowSettingsProfileWindow>,
    pserver_settings_windows: HashMap<HintedID, slint_ui::HereNowSettingsProfileServerWindow>,
}

thread_local! {
    static MAIN_UI: RefCell<MainUI> = RefCell::new(MainUI {
        main_window: None,
        profile_settings_windows: HashMap::new(),
        pserver_settings_windows: HashMap::new(),
    });
}

fn run_in_event_loop<F: FnOnce(&mut MainUI) + Send + 'static>(f: F) {
    slint::invoke_from_event_loop(move || {
        MAIN_UI.with(|main_ui| {
            let mut main_ui = main_ui.borrow_mut();
            f(&mut main_ui);
        });
    })
    .expect("invoking from event loop");
}

pub struct UILocalRecv(());

impl UILocalRecv {
    // separate out so we can handle a potential error at the caller
    fn apply(&self, msg: ui::ToUI) {
        use slint::*;
        match msg {
            ui::ToUI::ShowMainWindow => {
                run_in_event_loop(|main_ui| {
                    if let Some(main_w) = main_ui.main_window.as_mut() {
                        warn!("Showing main window");
                        main_w.show().expect("show window");
                    } else {
                        error!("No main window set");
                    }
                });
            }
            ui::ToUI::ShowPServerSettings(settings) => {
                error!("TODO: show pserver settings: {settings:#?}");
                // self
                // .pserver_settings_windows
                // .upgrade_in_event_loop(move |window: ui_settings_profile_server_window::HereNowSettingsProfileServerWindow| {
                //     warn!(?settings, "apply settings values");
                //     window.show().expect("show window");
                //     // windows_shown_clone.fetch_add(1, atomic::Ordering::SeqCst);
                //     window.set_server_url(match settings.server_url {
                //         ui::Setting::Value(v) => v.into(),
                //         ui::Setting::NoValue | ui::Setting::Unchanged => "".into(),
                //     });
                //     // need to reset since we're showing the new values
                //     window.invoke_reset();
                // })
            }
            ui::ToUI::NotifyProfileSettings(_profile_uid, _notification) => {
                // TODO: update some kind of shared models?
            }
            ui::ToUI::NotifyPServerSettings(_pserver_uid, _notification) => {
                // TODO: update some kind of shared models?
            }
            ui::ToUI::UpdateProfiles(profiles) => {
                error!("TODO: update profiles: {profiles:#?}");
            }
            ui::ToUI::HideProfileSettings(uid) => run_in_event_loop(move |main_ui| {
                if let Some(open) = main_ui.profile_settings_windows.get(&uid) {
                    open.invoke_close();
                } else {
                    warn!("no open profile settings window for {uid:?}");
                }
            }),
            ui::ToUI::HidePServerSettings(uid) => run_in_event_loop(move |main_ui| {
                if let Some(open) = main_ui.pserver_settings_windows.get(&uid) {
                    open.invoke_close();
                } else {
                    warn!("no open pserver settings window for {uid:?}");
                }
            }),
        }
    }
}

impl ui::SendToUI for UILocalRecv {
    #[tracing::instrument(skip(self))]
    fn send_all_to_ui(&self, msgs: Vec<ui::ToUI>) {
        eprintln!("send_all_to_ui:");
        for msg in msgs {
            eprintln!(" - {msg:?}");
            self.apply(msg);
        }
    }
}

pub fn main_blocking<T: ui::SendToExecutor>(
    send_to_executor: T,
    mut set_ui: impl FnMut(UILocalRecv),
) {
    let executor = Rc::new(send_to_executor);
    // let windows_shown = Arc::new(atomic::AtomicU8::new(0));

    let main_w: slint_ui::HereNowMainWindow = info_span!("create main window")
        .in_scope(|| slint_ui::HereNowMainWindow::new().expect("created window"));

    main_w.on_show_profile_settings({
        let executor_clone = executor.clone();
        move |uid| {
            info_span!("on show profile settings").in_scope(|| {
                match HintedID::try_from(uid.as_str()) {
                    Ok(uid) => {
                        executor_clone.send_to_executor(ui::ToExecutor::OpenPServerSettings(uid));
                    }
                    Err(err) => {
                        error!(?err, "invalid uid");
                    }
                }
            });
        }
    });

    main_w.on_close({
        let main_w_weak = main_w.as_weak();
        let executor_clone = executor.clone();
        // let windows_shown_clone = windows_shown.clone();
        move || {
            info_span!("closing application").in_scope(|| {
                main_w_weak
                    .upgrade()
                    .expect("in correct thread")
                    .hide()
                    .expect("hide window");
                executor_clone.send_to_executor(ui::ToExecutor::HidMainWindow);
            })
        }
    });

    // set main_w in MAIN_UI
    MAIN_UI.with(move |value| {
        value.borrow_mut().main_window = Some(main_w);
    });

    set_ui(UILocalRecv(()));

    i_slint_backend_selector::with_platform(|platform| {
        // importantly set this behavior so we don't exit if no windows are shown.
        // this allows us to keep the event loop running and show windows when we want.
        platform.set_event_loop_quit_on_last_window_closed(false);

        platform.run_event_loop()
    })
    .expect("run event loop for backend");

    tracing::error!("exited slint event loop unexpectedly");
}
