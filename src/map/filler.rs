use bevy::prelude::*;
use fujiformer_io::Filler;

use super::{ui::MapUiResources, Map};

#[derive(Bundle)]
pub struct FillerBundle {
    filler: Filler,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

pub fn add_on_map_load(
    mut commands: Commands,
    map_create: Query<&Map, Added<Map>>,
    res: Res<MapUiResources>,
) {
    if let Some(map) = map_create.iter().next() {
        for filler in map.0.fillers().iter() {
            let rect = filler.shape();
            commands.spawn_bundle(FillerBundle {
                filler: filler.clone(),
                sprite_bundle: SpriteBundle {
                    sprite: Sprite::new({
                        let size = rect.size();
                        Vec2::new(size.width() as f32, size.height() as f32)
                    }),
                    material: res.filler_color.clone(),
                    transform: {
                        let pos = rect.position();
                        Transform::from_xyz(pos.x() as f32, pos.y() as f32, 0.0)
                    },
                    ..Default::default()
                },
            });
        }
    }
}
