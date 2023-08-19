use std::rc::Rc;

use slint::ComponentHandle;
use tracing::*;

mod slint_ui {
    pub use ui_main_window::*;
    pub use ui_settings_window::*;
}

struct MainUI {
    main_window: slint::Weak<slint_ui::HereNowMainWindow>,
    settings_window: slint::Weak<slint_ui::HereNowSettingsWindow>,
}

// I need it so I can use it with the executor.
// Let's see if any errors occur...
unsafe impl Sync for MainUI {}

impl MainUI {
    // separate out so we can handle a potential error at the caller
    fn apply(&self, msg: ui::ToUI) -> Result<(), slint::EventLoopError> {
        match msg {
            ui::ToUI::ShowMainWindow => self.main_window.upgrade_in_event_loop(
                move |window: ui_main_window::HereNowMainWindow| {
                    window.show().expect("show window");
                },
            ),
            ui::ToUI::ShowSettings(settings) => self.settings_window.upgrade_in_event_loop(
                move |window: ui_settings_window::HereNowSettingsWindow| {
                    warn!(?settings, "apply settings values");
                    window.show().expect("show window");
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
                    window.hide().expect("hide window");
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

    let main_w: Rc<slint_ui::HereNowMainWindow> = Rc::new(
        info_span!("create main window")
            .in_scope(|| slint_ui::HereNowMainWindow::new().expect("created window")),
    );
    let settings_w: Rc<slint_ui::HereNowSettingsWindow> = Rc::new(
        info_span!("create settings window")
            .in_scope(|| slint_ui::HereNowSettingsWindow::new().expect("created window")),
    );

    set_ui(Box::new(MainUI {
        settings_window: settings_w.as_weak(),
        main_window: main_w.as_weak(),
    }));

    let settings_w_clone = settings_w.clone();
    settings_w.on_close({
        let executor_clone = executor.clone();
        move || {
            info_span!("on close settings").in_scope(|| {
                settings_w_clone.hide().expect("close window");
                executor_clone.send_to_executor(ui::ToExecutor::HidSettings);
            });
        }
    });

    let settings_w_clone = settings_w.clone();
    settings_w.on_apply({
        let executor_clone = executor.clone();
        let w = settings_w_clone;
        move || {
            info!("apply settings");
            executor_clone.send_to_executor(ui::ToExecutor::UpdateSettings(ui::Settings {
                server_url: ui::Setting::from_compared(
                    w.get_server_url(),
                    w.get_server_url_updated(),
                ),
                server_url_2: ui::Setting::from_compared(
                    w.get_server_url_2(),
                    w.get_server_url_2_updated(),
                ),
            }));
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

    let main_w_clone = main_w.clone();
    let executor_clone = executor.clone();
    main_w.on_close(move || {
        info_span!("closing application").in_scope(|| {
            main_w_clone.hide().expect("hide window");
            executor_clone.send_to_executor(ui::ToExecutor::HidMainWindow);
        })
    });

    slint::run_event_loop().expect("run event loop");

    // TODO: if all windows are hidden, then, the loop will exit...
    tracing::error!("unexpected exit of slint event loop");
}
