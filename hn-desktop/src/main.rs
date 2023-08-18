use std::sync::{Arc, Mutex};

fn main() {
    hn_tracing::expect_init_logger("hn-desktop");

    let ui_holder = UIHolderMutex::empty();
    let executor = hn_desktop_executor::main(Box::new(ui_holder.clone()));
    hn_desktop_ui::main_blocking(executor, move |ui| {
        ui_holder.set(ui);
    });
}

/// A mutex holding ui so we can swap the ui receiver out
#[derive(Clone)]
struct UIHolderMutex {
    ui: Arc<Mutex<Option<Box<dyn ui::SendToUI>>>>,
    queue: Arc<Mutex<Vec<ui::ToUI>>>,
}

impl UIHolderMutex {
    fn empty() -> Self {
        Self {
            ui: Arc::new(Mutex::new(None)),
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn set(&self, new_ui: Box<dyn ui::SendToUI>) {
        let mut ui = self.ui.lock().expect("can get a lock on ui");
        *ui = Some(new_ui);
        self.queue
            .lock()
            .expect("can get a lock on queue")
            .drain(..)
            .for_each(|msg| {
                ui.as_ref().expect("ui has been set").send_to_ui(msg);
            });
    }
}

impl ui::SendToUI for UIHolderMutex {
    fn send_to_ui(&self, msg: ui::ToUI) {
        let ui = self.ui.lock().expect("can get a lock on ui");
        match ui.as_ref() {
            Some(ui_sender) => ui_sender.send_to_ui(msg),
            None => {
                // add to queue
                self.queue
                    .lock()
                    .expect("can get a lock on queue")
                    .push(msg);
            }
        }
    }
}
