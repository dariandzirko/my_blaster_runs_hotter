use bevy::prelude::*;

use crate::{
    components::{Player, PlayerWeapon, WeaponData},
    constants::PLAYER_SPRITE_SCALE,
};
pub struct PlayerWeaponPlugin;

impl Plugin for PlayerWeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_startup_system(player_spawn_weapon_system)
            .add_system(weapon_follow_crosshair_system)
            // .add_system(weapon_attach_player_system)
            ;
    }
}

fn player_spawn_weapon_system(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("main_char/Gun/Main Gun/main gun_Gun_0.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(50.0, 50.0),
        1,
        1,
        None,
        Some(Vec2::new(-10.0, -5.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(PLAYER_SPRITE_SCALE)),
        ..default()
    };

    cmds.spawn(sprite)
        .insert(WeaponData::default())
        .insert(PlayerWeapon {
            is_player_weapon: true,
        });
}

// Why can't I put these together
fn weapon_follow_crosshair_system(
    mut weapon_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<PlayerWeapon>>,
    player_query: Query<&Player>,
) {
    if let Ok((mut weapon_transform, mut weapon_sprite)) = weapon_query.get_single_mut() {
        if let Ok(player) = player_query.get_single() {
            let angle = player.weapon_dir.y.atan2(player.weapon_dir.x);
            if angle.abs() > std::f32::consts::FRAC_PI_2 {
                weapon_sprite.flip_y = true;
            } else {
                weapon_sprite.flip_y = false;
            }

            weapon_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

fn weapon_attach_player_system(
    mut weapon_query: Query<&mut Transform, (With<PlayerWeapon>, Without<Player>)>,
    player_pos_query: Query<&Transform, (With<Player>, Without<PlayerWeapon>)>,
) {
    if let Ok(mut weapon_transform) = weapon_query.get_single_mut() {
        if let Ok(player_tf) = player_pos_query.get_single() {
            weapon_transform.translation.x = player_tf.translation.x;
            weapon_transform.translation.y = player_tf.translation.y;
            weapon_transform.translation.z = player_tf.translation.z + 1.0;
        }
    }
}
