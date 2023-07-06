use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window;

// mod ui {
//     slint::include_modules!();
// }

mod summary {
    slint::slint! {
        // import { ScreenShareGroup, HereNowScreenShareSelectorInner } from "ui/screen-share/screen-share-selector-inner.slint";
        export component SummaryWindow inherits Window {
            width: 600px;
            // in property <[ScreenShareGroup]> groups-model;
            callback choose-screen-share(string);
            callback close();
            Text {
                text: "Hello";
            }
            // HereNowScreenShareSelectorInner {
            //     groups-model: root.groups-model;
            //     choose-screen-share(id) => { root.choose-screen-share(id); }
            //     cancel => { root.close(); }
            // }
        }
    }
}

#[derive(Default)]
struct BasicApp {
    window: Window
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("Hello World!");
        self.window.show();
    }
}

fn main() -> Result<(), slint::PlatformError> {
    // let ui = summary::SummaryWindow::new()?;
    // // let ui = share_selector_window::new()?;

    // // let ui_handle = ui.as_weak();
    // // ui.on_request_increase_value(move || {
    // //     let ui = ui_handle.unwrap();
    // //     ui.set_counter(ui.get_counter() + 1);
    // // });

    App::new("com.colelawrence.herenow", BasicApp::default()).run();

    // ui.run()
    Ok(())
}
