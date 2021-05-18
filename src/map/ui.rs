use bevy::prelude::*;

pub struct MapUiPlugin;

impl Plugin for MapUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MapUiResources>();
        app.add_startup_system(ui_init.system());
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
    commands.spawn_bundle({
        let mut bundle = OrthographicCameraBundle::new_2d();
        bundle.transform.scale = Vec3::new(1.0, -1.0, 1.0);
        bundle
    });
}
