use std::rc::Rc;

use slint::{ComponentHandle, VecModel};
use tracing::*;

mod slint_ui {
    slint::include_modules!();
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
    screen_share_window: slint::Weak<screen_share::ScreenShareWindow>,
}

unsafe impl Sync for MainUI {}

impl ui::SendToUI for MainUI {
    #[tracing::instrument(skip(self))]
    fn send_to_ui(&self, msg: ui::ToUI) {
        info!(?msg, "send to ui");
        match msg {
            ui::ToUI::ShowMainWindow => self
                .window
                .upgrade_in_event_loop(|window| {
                    window.show().expect("show window");
                })
                .expect("upgrade in event loop"),
            ui::ToUI::ShowScreenShare => self
                .screen_share_window
                .upgrade_in_event_loop(|window| {
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

    a.on_start_screen_share({
        let groups_model = Rc::new(VecModel::<screen_share::ScreenShareGroup>::from(vec![]));
        let screen_share_window = Rc::<screen_share::ScreenShareWindow>::new(
            screen_share::ScreenShareWindow::new().unwrap(),
        );
        screen_share_window.set_groups_model(groups_model.clone().into());
        let screen_share_window_weak = screen_share_window.as_weak();
        let executor_clone = executor.clone();
        screen_share_window.on_close(move || {
            let _ = screen_share_window_weak.unwrap().hide();
            executor_clone.send_to_executor(ui::ToExecutor::HidScreenShare);
        });
        let screen_share_window_weak = screen_share_window.as_weak();
        screen_share_window.on_choose_screen_share(move |share_id| {
            println!("Add share {share_id:?}");
            let _ = screen_share_window_weak.unwrap().hide();
        });

        let main_ui = MainUI {
            screen_share_window: screen_share_window.as_weak(),
            window: a.as_weak(),
        };

        set_ui(Box::new(main_ui));

        std::mem::forget(screen_share_window);

        let executor_clone = executor.clone();
        move || {
            info_span!("on start screen share").in_scope(|| {
                executor_clone.send_to_executor(ui::ToExecutor::OpenScreenShare);
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

// fn create_share_group_for_slint(
//     // item: here_now_sdk::share_list::ShareListGroup,
//     icon_path: PathBuf,
// ) -> screen_share::ScreenShareGroup {
//     screen_share::ScreenShareGroup {
//         display_name: item.display_name.into(),
//         options: Rc::new(slint::VecModel::<screen_share::ScreenShareOption>::from(
//             item.items
//                 .into_iter()
//                 .map(|opt| screen_share::ScreenShareOption {
//                     display_name: opt.display_name.into(),
//                     id: opt.id.into(),
//                     // icon: ,
//                     // preview: ,
//                     ..Default::default()
//                 })
//                 .collect::<Vec<_>>(),
//         ))
//         .into(),
//         icon: slint::Image::load_from_path(&icon_path).expect("found image"),
//     }
// }
