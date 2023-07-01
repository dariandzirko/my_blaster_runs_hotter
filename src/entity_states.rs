use bevy::prelude::*;

pub trait SpriteLocation {
    fn location(&self) -> (usize, usize); //index_offset, row_length

    fn next_index(&self, curr_index: usize) -> usize {
        let (index_offset, row_length) = self.location();
        let mut next_index = curr_index.saturating_sub(index_offset);
        next_index = (next_index + 1) % row_length;
        return next_index + index_offset;
    }

    fn is_flip(&self) -> bool {
        false
    }
}

pub enum PlayerState {
    Run,
    Roll,
    Idle,
}

#[derive(Component)]
pub struct PlayerAnimationInfo {
    pub state: PlayerState,
    pub is_flip: bool,
}

impl SpriteLocation for PlayerAnimationInfo {
    fn location(&self) -> (usize, usize) {
        match &self.state {
            //Need to re-write all of these
            PlayerState::Run => (0 * 8, 8),
            PlayerState::Roll => (1 * 8, 6),
            PlayerState::Idle => (2 * 8, 5),
            _ => (0, 0),
        }
    }

    fn is_flip(&self) -> bool {
        return self.is_flip;
    }
}

pub enum SlimeState {
    Death,
    Run,
}

#[derive(Component)]
pub struct SlimeAnimationInfo {
    pub state: SlimeState,
}

impl SpriteLocation for SlimeAnimationInfo {
    fn location(&self) -> (usize, usize) {
        match &self.state {
            SlimeState::Death => (0 * 8, 13),
            SlimeState::Run => (1 * 8, 6),
            _ => (0, 0),
        }
    }
}
