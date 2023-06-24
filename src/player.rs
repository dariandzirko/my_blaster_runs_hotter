use bevy::prelude::*;
use bevy_rapier2d::dynamics::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::RigidBodyPosition;
use nalgebra::{vector, Vector2};

use crate::constants::*; //Should probably fix this, it's a little lazy

#[derive(Component)]
pub struct Player {
    move_dir: Vec2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_spawn_system)
            .add_system(keyboard_input_system)
            .add_system(player_move_system)
            .add_system(camera_follow_system);
    }
}

fn player_spawn_system(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;

    let texture_handle = asset_server.load("TeamGunner/CHARACTER_SPRITES/Blue/Blue_Soldier_50.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 8, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add the player sprite
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(PLAYER_SPRITE_SCALE)),
        ..default()
    };

    cmds.spawn(sprite)
        //Rigid Body
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(Player {
            move_dir: Vec2::ZERO,
        });
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut players: Query<&mut Player>) {
    let mut move_dir = Vec2::new(0.0, 0.0);

    // Y direction
    if keyboard_input.pressed(KeyCode::W) {
        move_dir.y = 1.0;
    } else if keyboard_input.pressed(KeyCode::S) {
        move_dir.y = -1.0;
    } else {
        move_dir.y = 0.0;
    }

    // X direction
    if keyboard_input.pressed(KeyCode::D) {
        move_dir.x = 1.0;
    } else if keyboard_input.pressed(KeyCode::A) {
        move_dir.x = -1.0;
    } else {
        move_dir.x = 0.0;
    }

    // This will not work for multiplayer
    for mut player in players.iter_mut() {
        player.move_dir = move_dir;
    }
}

fn player_move_system(mut players: Query<(&mut Velocity, &Player)>) {
    for (mut player_vel, player) in &mut players {
        *player_vel = Velocity::linear(player.move_dir * PLAYER_SPEED);
    }
}

fn camera_follow_system(
    player_pos_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    for player_pos in player_pos_query.iter() {
        for mut cam_pos in camera_query.iter_mut() {
            cam_pos.translation.x = player_pos.translation.x;
            cam_pos.translation.y = player_pos.translation.y;
        }
    }
}
