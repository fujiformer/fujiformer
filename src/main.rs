mod map;
mod ui;

use bevy::prelude::*;
use map::MapPlugin;
use ui::FfUiPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FfUiPlugin)
        .add_plugin(MapPlugin)
        .run();
}
