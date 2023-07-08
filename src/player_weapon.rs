use bevy::prelude::*;

use crate::{
    components::{Player, PlayerWeapon, WeaponData},
    constants::PLAYER_SPRITE_SCALE,
};
pub struct PlayerWeaponPlugin;

impl Plugin for PlayerWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(weapon_follow_crosshair_system);
    }
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
