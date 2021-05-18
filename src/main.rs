mod map;

use bevy::prelude::*;
use map::MapPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .run();
}
