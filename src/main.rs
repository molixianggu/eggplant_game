// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use eggplant_game::GamePlugin;
use bevy_rapier2d::prelude::*;


fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            title: "命运召唤 尔茄的精灵石".to_string(),
            canvas: Some("#main".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(GamePlugin)
        .run();
}
