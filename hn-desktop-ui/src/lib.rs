use std::rc::Rc;

use slint::ComponentHandle;
use tracing::*;
use ui_settings_window::Notification;

mod slint_ui {
    pub use ui_main_window::*;
    pub use ui_settings_window::*;
}

struct MainUI {
    // windows_shown: Arc<atomic::AtomicU8>,
    main_window: slint::Weak<slint_ui::HereNowMainWindow>,
    settings_window: slint::Weak<slint_ui::HereNowSettingsWindow>,
}

// I need it so I can use it with the executor.
// Let's see if any errors occur...
unsafe impl Sync for MainUI {}

impl MainUI {
    // separate out so we can handle a potential error at the caller
    fn apply(&self, msg: ui::ToUI) -> Result<(), slint::EventLoopError> {
        use slint::*;
        // let windows_shown_clone = self.windows_shown.clone();
        match msg {
            ui::ToUI::ShowMainWindow => self.main_window.upgrade_in_event_loop(
                move |window: ui_main_window::HereNowMainWindow| {
                    warn!("Showing main window");
                    window.show().expect("show window");
                    // windows_shown_clone.fetch_add(1, atomic::Ordering::SeqCst);
                },
            ),
            ui::ToUI::ShowSettings(settings) => self.settings_window.upgrade_in_event_loop(
                move |window: ui_settings_window::HereNowSettingsWindow| {
                    warn!(?settings, "apply settings values");
                    window.show().expect("show window");
                    // windows_shown_clone.fetch_add(1, atomic::Ordering::SeqCst);
                    window.set_server_url(match settings.server_url {
                        ui::Setting::Value(v) => v.into(),
                        ui::Setting::NoValue | ui::Setting::Unchanged => "".into(),
                    });
                    window.set_server_url_2(match settings.server_url_2 {
                        ui::Setting::Value(v) => v.into(),
                        ui::Setting::NoValue | ui::Setting::Unchanged => "".into(),
                    });
                    // need to reset since we're showing the new values
                    window.invoke_reset();
                },
            ),
            ui::ToUI::HideSettings => self.settings_window.upgrade_in_event_loop(
                move |window: ui_settings_window::HereNowSettingsWindow| {
                    window.invoke_close();
                },
            ),
            ui::ToUI::NotifySettings(notification) => self.settings_window.upgrade_in_event_loop(
                move |window: ui_settings_window::HereNowSettingsWindow| {
                    // let notifications_model: ModelRc<Notification> = window.get_notifications();
                    window.set_notifications(slint::ModelRc::new(slint::VecModel::from(vec![
                        Notification {
                            key: notification.key.clone().into(),
                            title: notification.title.clone().into(),
                            body: notification.body.clone().into(),
                        },
                        Notification {
                            key: std::format!("Test 2: {}", notification.key).into(),
                            title: notification.title.clone().into(),
                            body: notification.body.clone().into(),
                        },
                    ])));
                },
            ),
        }
    }
}

impl ui::SendToUI for MainUI {
    #[tracing::instrument(skip(self))]
    fn send_to_ui(&self, msg: ui::ToUI) {
        self.apply(msg).expect("apply message");
    }
}

pub fn main_blocking(
    send_to_executor: Box<dyn ui::SendToExecutor>,
    mut set_ui: impl FnMut(Box<dyn ui::SendToUI>),
) {
    let executor = Rc::new(send_to_executor);
    // let windows_shown = Arc::new(atomic::AtomicU8::new(0));

    let main_w: Rc<slint_ui::HereNowMainWindow> = Rc::new(
        info_span!("create main window")
            .in_scope(|| slint_ui::HereNowMainWindow::new().expect("created window")),
    );
    
    let settings_w: Rc<slint_ui::HereNowSettingsWindow> = Rc::new(
        info_span!("create settings window")
        .in_scope(|| slint_ui::HereNowSettingsWindow::new().expect("created window")),
    );

    // resets
    settings_w.set_notifications(slint::ModelRc::new(slint::VecModel::from(vec![])));

    set_ui(Box::new(MainUI {
        // windows_shown: windows_shown.clone(),
        settings_window: settings_w.as_weak(),
        main_window: main_w.as_weak(),
    }));

    settings_w.on_close({
        // let windows_shown_clone = windows_shown.clone();
        let settings_w_clone = settings_w.clone();
        let executor_clone = executor.clone();
        move || {
            info_span!("on close settings").in_scope(|| {
                // windows_shown_clone.fetch_sub(1, atomic::Ordering::SeqCst);
                settings_w_clone.hide().expect("close window");
                executor_clone.send_to_executor(ui::ToExecutor::HidSettings);
            });
        }
    });

    settings_w.on_apply({
        let settings_w_clone = settings_w.clone();
        let executor_clone = executor.clone();
        let w = settings_w_clone;
        move || {
            info!("apply settings");
            executor_clone.send_to_executor(ui::ToExecutor::UpdateSettings(
                ui::executor::UpdateSettings {
                    settings: ui::Settings {
                        server_url: ui::Setting::from_compared(
                            w.get_server_url(),
                            w.get_server_url_updated(),
                        ),
                        server_url_2: ui::Setting::from_compared(
                            w.get_server_url_2(),
                            w.get_server_url_2_updated(),
                        ),
                    },
                },
            ));
        }
    });

    main_w.on_show_settings({
        let executor_clone = executor.clone();
        move || {
            info_span!("on show settings").in_scope(|| {
                executor_clone.send_to_executor(ui::ToExecutor::OpenSettings);
            });
        }
    });

    main_w.on_close({
        let main_w_clone = main_w.clone();
        let executor_clone = executor.clone();
        // let windows_shown_clone = windows_shown.clone();
        move || {
            info_span!("closing application").in_scope(|| {
                // windows_shown_clone.fetch_sub(1, atomic::Ordering::SeqCst);
                main_w_clone.hide().expect("hide window");
                executor_clone.send_to_executor(ui::ToExecutor::HidMainWindow);
            })
        }
    });

    // loop {
    slint::run_event_loop().expect("run event loop");
    // TODO: if all windows are hidden, then, the loop will exit...
    tracing::error!("exited slint event loop because all windows are closed");

    // block on waiting for windows_shown to be true ?
    // while windows_shown.load(atomic::Ordering::SeqCst) == 0 {
    //     tracing::warn!("waiting for windows to be shown");
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    // }
    // }
}
