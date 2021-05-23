use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::ui::{ScrollInteraction, UiEventStage};

pub struct MapUiPlugin;

impl Plugin for MapUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MapUiResources>()
            .add_startup_system(ui_init.system())
            .add_system(apply_scroll.system().after(UiEventStage))
            .insert_resource(MouseMapPosition(None))
            .add_system(update_mouse_map_position.system().label(UiEventStage));
    }
}

pub struct MapUiResources {
    pub filler_color: Handle<ColorMaterial>,
}

impl FromWorld for MapUiResources {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        MapUiResources {
            filler_color: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
        }
    }
}

pub fn ui_init(mut commands: Commands) {
    commands.spawn_bundle(MapCameraBundle::default());
}

pub struct MapCamera;

#[derive(Bundle)]
struct MapCameraBundle {
    map_camera: MapCamera,
    scroll_interaction: ScrollInteraction,
    #[bundle]
    camera: OrthographicCameraBundle,
}

impl Default for MapCameraBundle {
    fn default() -> Self {
        let mut camera = OrthographicCameraBundle::new_2d();
        camera.transform.scale = Vec3::new(1.0, -1.0, 1.0);
        MapCameraBundle {
            map_camera: MapCamera,
            scroll_interaction: Default::default(),
            camera,
        }
    }
}

fn apply_scroll(
    mut camera: Query<(&mut Transform, &mut ScrollInteraction), Changed<ScrollInteraction>>,
    mouse_map_position: Res<MouseMapPosition>,
) {
    for (mut transform, mut scroll) in camera.iter_mut() {
        if let Some(ref mut scroll) = scroll.0 {
            // It's not possible for this to fail as `ScrollInteraction` has just changed.
            let map_position = mouse_map_position.0.unwrap();

            let scale_factor = if scroll.y > 0.0 { 0.8 } else { 1.0 / 0.8 };
            let old_scale = transform.scale.x;
            let new_scale = old_scale * scale_factor;
            transform.scale.x = new_scale;
            transform.scale.y = -new_scale;
            let ratio = new_scale / old_scale;
            transform.translation = (map_position * (1.0 - ratio)
                + transform.translation.truncate() * ratio)
                .extend(999.9);
        }
    }
}

pub struct MouseMapPosition(Option<Vec2>);

fn update_mouse_map_position(
    windows: Res<Windows>,
    mut motion_event: EventReader<MouseMotion>,
    mut mouse_map_position: ResMut<MouseMapPosition>,
    camera: Query<&Transform, With<MapCamera>>,
) {
    if motion_event.iter().next().is_some() {
        let window = windows.get_primary().unwrap();
        mouse_map_position.0 = window
            .cursor_position()
            .map(|pos| {
                let pos = Vec2::new(
                    pos.x as f32 - window.width() as f32 * 0.5,
                    pos.y as f32 - window.height() as f32 * 0.5,
                );
                camera.iter().next().map(|transform| {
                    (transform.compute_matrix() * pos.extend(0.0).extend(1.0))
                        .truncate()
                        .truncate()
                })
            })
            .flatten()
    }
}
