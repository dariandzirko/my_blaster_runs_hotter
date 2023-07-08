use crate::constants::*;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::*;
use bevy_rapier2d::prelude::*;
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>()
            .add_system(projectile_understanding_system);
    }
}

pub struct SpawnProjectileEvent {
    pub position: Vec3,
    pub direction: Vec2,
    pub collision_group: u32,
}

pub fn projectile_understanding_system(
    mut cmd: Commands,
    mut projectile_reader: EventReader<SpawnProjectileEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    for event in projectile_reader.iter() {
        spawn_projectile(&mut cmd, event, &mut texture_atlases, &asset_server);
    }
}

pub fn spawn_projectile(
    cmd: &mut Commands,
    projectile_info: &SpawnProjectileEvent,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
) {
    let texture_handle = asset_server.load("main_char/Gun/Bullet/bullet_projectile_0.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(50.0, 50.0),
        1,
        1,
        None,
        Some(Vec2::new(0.0, -5.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let angle = projectile_info
        .direction
        .y
        .atan2(projectile_info.direction.x);

    let transform = Transform::default()
        .with_rotation(Quat::from_rotation_z(angle))
        .with_scale(Vec3::splat(PLAYER_SPRITE_SCALE))
        .with_translation(Vec3 {
            x: projectile_info.position.x,
            y: projectile_info.position.y,
            z: 0.0,
        });

    // Add the player sprite
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        ..default()
    };

    cmd.spawn(sprite)
        .insert(Collider::cuboid(
            DEFAULT_BULLET_WIDTH,
            DEFAULT_BULLET_HEIGHT,
        ))
        .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1))
        .insert(Dominance::group(-1))
        //Rigid Body
        .insert(RigidBody::KinematicVelocityBased)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::linear(projectile_info.direction * BASE_GUN_SPEED))
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(transform);
}
