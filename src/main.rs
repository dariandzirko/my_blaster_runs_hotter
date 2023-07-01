use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier2d::prelude::*;
use constants::{GRID_LENGTH, GRID_THICKNESS, MAP_SIZE};
use player::PlayerPlugin;
use player_weapon::PlayerWeaponPlugin;
use projectile::ProjectilePlugin;

mod animation;
mod components;
mod constants;
mod entity_states;
mod player;
mod player_weapon;
mod projectile;

use crate::animation::AnimationPlugin;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(PlayerWeaponPlugin)
        .add_plugin(ProjectilePlugin)
        .add_system(hide_cursor)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());

    //Need to add the rest of the thickness to the grid because I am off by a little

    //Map definition
    //Horizontal lines, making a MAP_SIZE number of very long horizontal lines, length being GRID_LENGTH * MAP_SIZE
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
    //Vertical lines, , making a MAP_SIZE number of very long vertical lines, length being GRID_LENGTH * MAP_SIZE
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

fn hide_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Confined;
    }
}
