use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{AnimationTimer, Player, Slime},
    constants::{
        PLAYER_ANIMATION_TIMER, PLAYER_SPRITE_SCALE, SLIME_HEIGHT, SLIME_SPEED, SLIME_WIDTH,
    },
    entity_states::{SlimeAnimationInfo, SlimeState},
};

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSlimeEvent>()
            .add_system(slime_movement_system)
            //.add_system(slime_animation_system)
            .add_system(spawn_slime_system);
    }
}

pub struct SpawnSlimeEvent {
    pub position: Vec3,
}

pub fn spawn_slime_system(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut spawn_slime_event: EventReader<SpawnSlimeEvent>,
) {
    for event in spawn_slime_event.iter() {
        spawn_slime(&mut cmd, event, &mut texture_atlases, &asset_server);
    }
}

pub fn spawn_slime(
    cmd: &mut Commands,
    slime_info: &SpawnSlimeEvent,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
) {
    let texture_handle = asset_server.load("slime_sprite_sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 13, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add the player sprite
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        ..default()
    };

    let transform = Transform::default()
        .with_scale(Vec3::splat(PLAYER_SPRITE_SCALE))
        .with_translation(slime_info.position);

    cmd.spawn(sprite)
        .insert(Collider::cuboid(SLIME_WIDTH, SLIME_HEIGHT))
        //Rigid Body
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::zero())
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(transform)
        .insert(Slime {
            move_dir: Vec2::ZERO,
        })
        .insert(SlimeAnimationInfo {
            state: SlimeState::Run,
        })
        .insert(AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_TIMER,
            TimerMode::Repeating,
        )));
}

pub fn slime_movement_system(
    player: Query<&Transform, With<Player>>,
    mut slimes: Query<(&Transform, &mut Slime, &mut Velocity)>,
) {
    if let Ok(player_transform) = player.get_single() {
        for (slime_transform, mut slime, mut slime_velocity) in slimes.iter_mut() {
            slime.move_dir = Vec2::new(
                player_transform.translation.x - slime_transform.translation.x,
                player_transform.translation.y - slime_transform.translation.y,
            );
            slime_velocity.linvel = slime.move_dir.normalize() * SLIME_SPEED;
        }
    }
}

pub fn slime_animation_system(mut slimes: Query<&mut SlimeAnimationInfo, With<Slime>>) {
    for mut slime_animation_info in slimes.iter_mut() {
        slime_animation_info.state = SlimeState::Run;
    }
}
