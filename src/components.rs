use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Crosshair {
    pub is_crosshair: bool,
}

#[derive(Component)]
pub struct Player {
    pub move_dir: Vec2,
    pub weapon_dir: Vec2,
    pub is_invincible: bool,
    pub roll_timer: Timer,
}

#[derive(Component)]
pub struct PlayerWeapon {
    pub is_player_weapon: bool,
}

#[derive(Component)]
pub struct Slime {
    pub move_dir: Vec2,
}

#[derive(Component)]
pub struct WeaponData {
    pub firing: bool,
    pub fire_rate_timer: Timer,
    pub damage: u32,
}

//Default is qwark's ranged weapon
impl Default for WeaponData {
    fn default() -> Self {
        Self {
            firing: false,
            fire_rate_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            damage: 1,
        }
    }
}
