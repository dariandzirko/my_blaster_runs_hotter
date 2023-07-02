use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::dynamics::*;
use bevy_rapier2d::prelude::*;
use nalgebra::{vector, Vector2};

use crate::components::*;
use crate::constants::*;
use crate::entity_states::{PlayerAnimationInfo, PlayerState};
use crate::projectile::SpawnProjectileEvent; //Should probably fix this, it's a little lazy

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((player_spawn_system, spawn_crosshair_system))
            .add_systems((
                keyboard_input_system,
                player_move_system,
                camera_follow_system,
                mouse_input_system,
                crosshair_follow_system,
                player_animation_state_system,
            ));
    }
}

fn player_spawn_system(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;

    let texture_handle = asset_server.load("main_char/main_char_sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(50.0, 50.0),
        8,
        3,
        None,
        Some(Vec2::new(0.0, 3.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add the player sprite
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(PLAYER_SPRITE_SCALE)),
        ..default()
    };

    let texture_handle = asset_server.load("main_char/Gun/Main Gun/main gun_Gun_0.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(50.0, 50.0),
        1,
        1,
        None,
        Some(Vec2::new(-7.0, 0.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let weapon_sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }),
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
            weapon_dir: Vec2::ZERO,
            is_invincible: false,
        })
        .insert(PlayerAnimationInfo {
            state: PlayerState::Idle,
            is_flip: false,
        })
        .insert(AnimationTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .with_children(|parent| {
            parent
                .spawn(weapon_sprite)
                .insert(WeaponData::default())
                .insert(PlayerWeapon {
                    is_player_weapon: true,
                });
        });
}

// I might want to break states out but for now this should be okay
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

    if keyboard_input.pressed(KeyCode::LControl) {
        // send a roll event

        // I should just send a roll event with this player, so I can add more speed and I frames to the player
    }

    // This will not work for multiplayer, need a player_handle input to distinguish between players, maybe break this out
    for mut player in players.iter_mut() {
        player.move_dir = move_dir;
    }
}

fn player_animation_state_system(mut players: Query<(&Player, &mut PlayerAnimationInfo)>) {
    if let Ok((player, mut player_animation_info)) = players.get_single_mut() {
        if player.move_dir == Vec2::ZERO {
            player_animation_info.state = PlayerState::Idle;
        } else {
            player_animation_info.state = PlayerState::Run;
        }

        //I am not sure this would work with my current idea of the roll animation
        if player.weapon_dir.x < 0.0 {
            player_animation_info.is_flip = true;
        } else {
            player_animation_info.is_flip = false;
        }
    }
}

fn player_move_system(mut players: Query<(&mut Velocity, &mut Transform, &Player)>) {
    for (mut player_vel, player_tf, player) in &mut players {
        *player_vel = Velocity::linear(player.move_dir * PLAYER_SPEED);

        if player_tf.translation.x.abs() >= (MAP_SIZE / 2) as f32 * GRID_LENGTH {
            player_vel.linvel.x = -5.0 * player_tf.translation.x.signum() * PLAYER_SPEED;
        }
        if player_tf.translation.y.abs() >= (MAP_SIZE / 2) as f32 * GRID_LENGTH {
            player_vel.linvel.y = -5.0 * player_tf.translation.y.signum() * PLAYER_SPEED;
        }
    }
}

fn mouse_input_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_pos_query: Query<(&Transform, &mut Player)>,
    mut spawn_projectile_events: EventWriter<SpawnProjectileEvent>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    for (player_tf, mut player) in player_pos_query.iter_mut() {
        if let Ok(window) = window_query.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                player.weapon_dir = Vec2::new(
                    cursor_pos.x - window.width() / 2.0 - player_tf.translation.x,
                    cursor_pos.y - window.height() / 2.0 - player_tf.translation.y,
                )
                .normalize();
            }
            if mouse_buttons.just_pressed(MouseButton::Left) {
                spawn_projectile_events.send(SpawnProjectileEvent {
                    position: player_tf.translation,
                    direction: player.weapon_dir,
                    collision_group: 1,
                })
            }
        }
    }
}

// Can even make this a resource and add this to some ui stuff. So I can select the menus with a fun little crosshair
fn spawn_crosshair_system(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("main_char/Gun/Crosshair/crosshair_Crosshair_0.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add the player sprite
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(PLAYER_SPRITE_SCALE)),
        ..default()
    };

    cmds.spawn(sprite).insert(Crosshair { is_crosshair: true });
}

// This is a little naive and maybe I should figure out how to change the cursor that is rendered to the screen
fn crosshair_follow_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut crosshair_query: Query<&mut Transform, With<Crosshair>>,
) {
    if let Ok(window) = window_query.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(mut crosshair) = crosshair_query.get_single_mut() {
                crosshair.translation.x = cursor_pos.x - window.width() / 2.0;
                crosshair.translation.y = cursor_pos.y - window.height() / 2.0;
            }
        }
    }
}

fn camera_follow_system(
    player_pos_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_pos) = player_pos_query.get_single() {
        for mut cam_pos in camera_query.iter_mut() {
            cam_pos.translation.x = player_pos.translation.x;
            cam_pos.translation.y = player_pos.translation.y;
        }
    }
}
