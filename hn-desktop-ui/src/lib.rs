use std::rc::Rc;

use slint::ComponentHandle;
use tracing::*;

mod slint_ui {
    pub use ui_main_window::*;
    pub use ui_settings_window::*;
}

mod screen_share {
    slint::slint! {
        import { ScreenShareGroup, HereNowScreenShareSelectorInner } from "ui/screen-share/screen-share-selector-inner.slint";
        export component ScreenShareWindow inherits Window {
            width: 600px;
            in property <[ScreenShareGroup]> groups-model;
            callback choose-screen-share(string);
            callback close();
            HereNowScreenShareSelectorInner {
                groups-model: root.groups-model;
                choose-screen-share(id) => { root.choose-screen-share(id); }
                cancel => { root.close(); }
            }
        }
    }
}

struct MainUI {
    main_window: slint::Weak<slint_ui::HereNowMainWindow>,
    settings_window: slint::Weak<slint_ui::HereNowSettingsWindow>,
}

unsafe impl Sync for MainUI {}

impl ui::SendToUI for MainUI {
    #[tracing::instrument(skip(self))]
    fn send_to_ui(&self, msg: ui::ToUI) {
        info!(?msg, "send to ui");
        match msg {
            ui::ToUI::ShowMainWindow => self
                .main_window
                .upgrade_in_event_loop(move |window| {
                    window.show().expect("show window");
                })
                .expect("upgrade in event loop"),
            ui::ToUI::ShowSettings(settings) => self
                .settings_window
                .upgrade_in_event_loop(move |window| {
                    warn!(?settings, "apply settings values");
                    window.show().expect("show window");
                })
                .expect("upgrade in event loop"),
        }
    }
}

pub fn main_blocking(
    send_to_executor: Box<dyn ui::SendToExecutor>,
    mut set_ui: impl FnMut(Box<dyn ui::SendToUI>),
) {
    let executor = Rc::new(send_to_executor);

    let main_w = Rc::new(
        info_span!("create main window")
            .in_scope(|| slint_ui::HereNowMainWindow::new().expect("created window")),
    );
    let settings_w = Rc::new(
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

    // TODO: if all windows are hidden, then, the loopp will exit...
    tracing::error!("unexpected exit of slint event loop");
}
