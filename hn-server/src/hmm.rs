use std::{path::PathBuf, time::Duration};

use crate::prelude::*;
use shipyard::{EntityId, World};
use shipyard_app::{App, Plugin};
use watchable::{Watchable, Watcher};

mod config_plugins;

struct Settings {
    config_files_w: Watchable<usize>,
    world: shipyard::World,
}

pub async fn start(root: Watcher<()>) {
    let config_files_w = Watchable::new(0usize);
    let world = shipyard::World::new();
    let settings = Arc::new(Settings {
        config_files_w,
        world,
    });

    // world.add_unique(component);

    // let other_h = tokio::spawn(other(settings.clone()));

    for () in root {
        // wait for shutdown
        return;
    }
}

