mod ui {
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

use std::{path::PathBuf, rc::Rc, sync::Arc};

use slint::{ComponentHandle, VecModel};

pub fn slint_main() {
    let a = Arc::<ui::HereNowMainWindow>::new(ui::HereNowMainWindow::new().unwrap());
    a.on_start_screen_share({
        let groups_model = Rc::new(VecModel::<screen_share::ScreenShareGroup>::from(vec![]));
        let screen_share_window = Arc::<screen_share::ScreenShareWindow>::new(
            screen_share::ScreenShareWindow::new().unwrap(),
        );
        screen_share_window.set_groups_model(groups_model.clone().into());
        let screen_share_window_weak = screen_share_window.as_weak();
        screen_share_window.on_close(move || {
            let _ = screen_share_window_weak.unwrap().hide();
        });
        let screen_share_window_weak = screen_share_window.as_weak();
        screen_share_window.on_choose_screen_share(move |share_id| {
            println!("Add share {share_id:?}");
            let _ = screen_share_window_weak.unwrap().hide();
        });

        // let groups_model = groups_model.clone();
        move || {
            println!("Start screen share");

            // let mut share_plugin = ShareMediaPlugin::default();
            // share_plugin.add_folder(
            //     "Samples",
            //     PathBuf::from("./plugins/here-now-share-media/samples"),
            // );
            // // todo: some kind of parallelism?
            // match share_plugin.share_list(&ShareListOptions {}) {
            //     Ok(list) => {
            //         for item in list.groups {
            //             let icon_path = PathBuf::from("ui/none.png");
            //             groups_model.push(create_share_group_for_slint(item, icon_path))
            //         }
            //     }
            //     Err(err) => eprintln!("Failed to get share list: {err:?}"),
            // }
            // TODO: load the screen share options from plugins
            screen_share_window.show().unwrap();
            let screen_share_window_weak = screen_share_window.as_weak();
            screen_share_window.on_close(move || {
                let _ = screen_share_window_weak.unwrap().hide();
            });
        }
    });
    a.on_close(|| {
        std::process::exit(0);
    });
    // a.set_groups_model(groups_model.into());
    a.run().unwrap();
    println!("Done!");
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
