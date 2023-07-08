use bevy::prelude::*;

use crate::slime::SpawnSlimeEvent;

pub struct SpawnManagerPlugin;

impl Plugin for SpawnManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(slime_spawn_system);
    }
}

pub fn slime_spawn_system(mut spawn_slime_events: EventWriter<SpawnSlimeEvent>) {
    spawn_slime_events.send(SpawnSlimeEvent {
        position: Vec3 {
            x: 50.0,
            y: 50.0,
            z: 1.0,
        },
    });
}
