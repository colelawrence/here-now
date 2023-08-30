mod ui_holder_mutex;

fn main() {
    hn_tracing::expect_init_logger("hn-desktop");

    let ui_holder = ui_holder_mutex::UIHolderMutex::empty();
    let executor = hn_desktop_executor::main(Box::new(ui_holder.clone()));
    hn_desktop_ui::main_blocking(executor, move |ui| {
        ui_holder.set(ui);
    });
}

#[test]
#[ignore]
fn test_startup_time() {
    let start_time = std::time::Instant::now();
    hn_tracing::expect_init_logger("hn-desktop-startup-time");
    let ui_holder = ui_holder_mutex::UIHolderMutex::empty();
    let executor = hn_desktop_executor::main(Box::new(ui_holder.clone()));
    hn_desktop_ui::main_blocking(executor, move |_| {
        // print time elapsed since start
        println!("startup time: {:?}", start_time.elapsed());
        std::process::exit(0);
    });
}
