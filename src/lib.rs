mod trainer;

use bevy::prelude::*;

pub fn run_game() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Staff Trainer".into(),
            resolution: (1200., 540.).into(),
            ..default()
        }),
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(trainer::TrainerPlugin)
        .run();
}

#[bevy_main]
fn main() {
    run_game();
}