use bevy::prelude::*;
use fujiformer_io::Screen;

use super::{ui::MapUiResources, Map};

#[derive(Bundle)]
pub struct ScreenBundle {
    screen: Screen,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

pub fn add_on_map_load(
    mut commands: Commands,
    map_create: Query<&Map, Added<Map>>,
    res: Res<MapUiResources>,
) {
    if let Some(map) = map_create.iter().next() {
        for screen in map.0.screens().iter() {
            let rect = screen.shape();
            let size = rect.size();
            let width = size.width() as f32;
            let height = size.height() as f32;
            commands.spawn_bundle(ScreenBundle {
                screen: screen.clone(),
                sprite_bundle: SpriteBundle {
                    sprite: Sprite::new(Vec2::new(width, height)),
                    material: res.screen_color.clone(),
                    transform: {
                        let pos = rect.position();
                        Transform::from_xyz(
                            pos.x() as f32 + width * 0.5,
                            pos.y() as f32 + height * 0.5,
                            0.0,
                        )
                    },
                    ..Default::default()
                },
            });
        }
    }
}
