use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameWinner {
    None,
    Player1,
    Player2,
}

#[derive(Default)]
pub struct Game {
    pub score: i32,
    pub winner: GameWinner,
}

impl Default for GameWinner {
    fn default() -> Self {
        return GameWinner::None
    }
}

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}