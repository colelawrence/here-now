mod ui_holder_mutex;

fn main() {
    hn_tracing::expect_init_logger("hn-desktop");

    let ui_holder = ui_holder_mutex::UIHolderMutex::empty();
    let executor = hn_desktop_executor::main(ui_holder.clone());
    hn_desktop_ui::main_blocking(executor, move |ui| {
        ui_holder.set(Box::new(ui));
    });
}
