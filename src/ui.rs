use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::map::MapCamera;

pub struct FfUiPlugin;

impl Plugin for FfUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearColor(Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }))
        .add_system(ui_event_handler.system().label(UiEventStage));
    }
}

#[derive(SystemLabel, Eq, PartialEq, Hash, Clone, Debug)]
pub struct UiEventStage;

fn ui_event_handler(
    mut scroll: EventReader<MouseWheel>,
    mut map: Query<&mut ScrollInteraction, With<MapCamera>>,
) {
    for event in scroll.iter() {
        if let Ok(mut scroll_event) = map.single_mut() {
            scroll_event.0 = Some(event.clone());
        }
    }
}

#[derive(Default)]
pub struct ScrollInteraction(pub Option<MouseWheel>);
