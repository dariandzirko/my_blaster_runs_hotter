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
}

pub enum EnemyState {
    Death,
    Run,
    Idle,
    Attack,
}

#[derive(Component)]
pub struct EnemyAnimationInfo {
    pub state: EnemyState,
    pub is_flip: bool,
}

impl SpriteLocation for EnemyAnimationInfo {
    fn location(&self) -> (usize, usize) {
        match &self.state {
            EnemyState::Death => (0 * 8, 8),
            EnemyState::Run => (1 * 8, 6),
            EnemyState::Idle => (4 * 8, 5),
            EnemyState::Attack => (4 * 8, 5),
            _ => (0, 0),
        }
    }

    fn is_flip(&self) -> bool {
        return self.is_flip;
    }
}
