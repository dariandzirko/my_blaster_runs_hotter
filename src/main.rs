use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use constants::{GRID_LENGTH, GRID_THICKNESS, MAP_SIZE, PLAYER_WIDTH};
use player::PlayerPlugin;

mod constants;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());

    //Map definition
    // Horizontal lines
    for i in 0..=MAP_SIZE {
        cmds.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0.,
                (i as f32 - (MAP_SIZE / 2) as f32) * GRID_LENGTH,
                0.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_LENGTH * MAP_SIZE as f32, GRID_THICKNESS)),
                ..default()
            },
            ..default()
        });
    }

    for i in 0..=MAP_SIZE {
        cmds.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                (i as f32 - (MAP_SIZE / 2) as f32) * GRID_LENGTH,
                0.,
                0.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_THICKNESS, GRID_LENGTH * MAP_SIZE as f32)),
                ..default()
            },
            ..default()
        });
    }
}
