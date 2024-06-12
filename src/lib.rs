mod trainer;

use bevy::prelude::*;

pub fn run_game() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_plugins(trainer::TrainerPlugin)
        .run();
}

#[bevy_main]
fn main() {
    run_game();
}