use std::rc::Rc;

use slint::{ComponentHandle, VecModel};
use tracing::*;

mod slint_ui {
    slint::include_modules!();
}

mod settings {
    slint::slint! {
        import { HereNowSettingsWindow } from "ui/settings_window.slint";
        export component SettingsWindow inherits HereNowSettingsWindow {}
    }
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
    window: slint::Weak<slint_ui::HereNowMainWindow>,
    settings_window: slint::Weak<settings::SettingsWindow>,
}

unsafe impl Sync for MainUI {}

impl ui::SendToUI for MainUI {
    #[tracing::instrument(skip(self))]
    fn send_to_ui(&self, msg: ui::ToUI) {
        info!(?msg, "send to ui");
        match msg {
            ui::ToUI::ShowMainWindow => self
                .window
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
    let a = info_span!("create main window").in_scope(|| {
        Rc::<slint_ui::HereNowMainWindow>::new(
            slint_ui::HereNowMainWindow::new().expect("created window"),
        )
    });

    let executor = Rc::new(send_to_executor);
    let settings_window = Rc::new(
        info_span!("create settings window")
            .in_scope(|| settings::SettingsWindow::new().expect("created window")),
    );
    let main_ui = MainUI {
        settings_window: settings_window.as_weak(),
        window: a.as_weak(),
    };
    set_ui(Box::new(main_ui));

    a.on_show_settings({
        let executor_clone = executor.clone();
        move || {
            info_span!("on show settings").in_scope(|| {
                executor_clone.send_to_executor(ui::ToExecutor::OpenSettings);
            });
        }
    });

    let a_clone = a.clone();
    let executor_clone = executor.clone();
    a.on_close(move || {
        info_span!("closing application").in_scope(|| {
            a_clone.hide().expect("hide window");
            executor_clone.send_to_executor(ui::ToExecutor::HidMainWindow);
        })
    });

    slint::run_event_loop().expect("run event loop");

    tracing::error!("unexpected exit of slint event loop");
}
