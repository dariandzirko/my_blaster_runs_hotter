use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::AnimationTimer,
    entity_states::{EnemyAnimationInfo, PlayerAnimationInfo, SpriteLocation},
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation_system::<PlayerAnimationInfo>);
        //app.add_system(animation_system::<EnemyAnimationInfo>);
    }
}

fn animation_system<T: Component + SpriteLocation>(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &T)>,
) {
    for (mut timer, mut sprite, info) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = info.next_index(sprite.index)
        }

        //This may be a naive approach but it might jsut work for what we need, and it would be tedious to create 2 copies of every sprite
        if info.is_flip() {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }
}
