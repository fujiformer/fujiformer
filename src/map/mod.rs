mod filler;
mod ui;

use std::{fs::File, io::BufReader};

use bevy::prelude::*;
use fujiformer_io::CelesteMap;

use self::ui::MapUiPlugin;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MapUiPlugin);
        app.add_startup_system(load_map.system());
        app.add_system(filler::add_on_map_load.system());
    }
}

pub struct Map(CelesteMap);

fn load_map(mut commands: Commands) {
    let map_path = std::env::args().nth(1).unwrap();
    let map_file = BufReader::new(File::open(map_path).unwrap());
    let map = CelesteMap::read(map_file).unwrap();
    commands.spawn().insert(Map(map));
}
