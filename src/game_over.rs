use bevy::prelude::*;

use crate::pong;

pub fn setup_game_over(mut commands: Commands, windows: Res<Windows>, asset_server: Res<AssetServer>, game: Res<pong::Game>) {
    let window = windows.primary();
   
    // ui
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                if game.winner == pong::GameWinner::Player1 { format!("You won! Score: {}", game.score) } else { format!("You lost! Score: {}", game.score) },
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(100.0),
                    ..default()
                },
                align_self: AlignSelf::Center,
                ..default()
            },
            ..default()
        });
}

pub fn gameover_keyboard(mut state: ResMut<State<pong::GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let _ = state.overwrite_set(pong::GameState::Playing);
    }
}