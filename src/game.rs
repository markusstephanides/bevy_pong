use bevy::prelude::*;

use crate::pong;

const PADDLE_HEIGHT_FACTOR: f32 = 0.2;

pub fn setup_game(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<pong::Game>,
) {
    let window = windows.primary();

    game.winner = pong::GameWinner::None;
    game.score = 0;

    // ball
    const BALL_HEIGHT: f32 = 10.0;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, BALL_HEIGHT, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Ball)
        .insert(Direction {
            value: Vec3::new(-1.0, 1.0, 0.0),
        })
        .insert(Speed(400.0))
        .insert(SpeedFactor { value: 1.0 });

    // player paddle
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, window.height() * PADDLE_HEIGHT_FACTOR, 0.0),
                translation: Vec3::new(window.width() * -0.5 + 30.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(PlayerPaddle)
        .insert(Speed(700.0))
        .insert(BorderRestriction {
            border_offset: 20.0,
        })
        .insert(DirectionInverter {
            axis: Vec3::new(1.0, 0.0, 0.0),
            awards_points: true,
        })
        .insert(Direction {
            value: Vec3::new(0.0, 0.0, 0.0),
        });

    // opponent paddle
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, window.height() * PADDLE_HEIGHT_FACTOR, 0.0),
                translation: Vec3::new(window.width() * 0.5 - 30.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(OpponentPaddle)
        .insert(Speed(700.0))
        .insert(BorderRestriction {
            border_offset: 20.0,
        })
        .insert(DirectionInverter {
            axis: Vec3::new(1.0, 0.0, 0.0),
            awards_points: false,
        })
        .insert(Direction {
            value: Vec3::new(0.0, 0.0, 0.0),
        });

    // ui
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "Score:",
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 20.0,
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
                    top: Val::Px(5.0),
                    ..default()
                },
                align_self: AlignSelf::Center,
                ..default()
            },
            ..default()
        })
        .insert(ScoreText);
}

pub fn update_player_paddle(
    mut query: Query<(&PlayerPaddle, &mut Direction)>,
    keys: Res<Input<KeyCode>>,
) {
    let (_paddle, mut direction) = query.single_mut();

    if keys.pressed(KeyCode::W) {
        direction.value.y = 1.0;
    } else if keys.pressed(KeyCode::S) {
        direction.value.y = -1.0;
    } else {
        direction.value.y = 0.0;
    }
}

pub fn update_opponent_paddle(
    mut paddle_query: Query<(&OpponentPaddle, &Speed, &Transform, &mut Direction)>,
    ball_query: Query<(&Ball, &Transform, &Direction), Without<OpponentPaddle>>,
) {
    let (_paddle, speed, paddle_transform, mut direction) = paddle_query.single_mut();
    let (_ball, ball_transform, ball_direction) = ball_query.single();

    if ball_transform.translation.x <= 0.0 || ball_direction.value.x < 0.0 {
        // move paddle towards the center
        if paddle_transform.translation.y < -10.0 {
            direction.value.y = 1.0;
        } else if paddle_transform.translation.y > 10.0 {
            direction.value.y = -1.0;
        } else {
            direction.value.y = 0.0;
        }
    } else {
        // Predict
        if paddle_transform.translation.y < ball_transform.translation.y {
            direction.value.y = 1.0;
        } else if paddle_transform.translation.y > ball_transform.translation.y {
            direction.value.y = -1.0;
        } else {
            direction.value.y = 0.0;
        }
    }
}

pub fn update_paddle_restrictor(
    mut query: Query<(&BorderRestriction, &mut Transform)>,
    windows: Res<Windows>,
) {
    let window = windows.primary();

    for (_restriction, mut transform) in query.iter_mut() {
        let paddle_height: f32 = transform.scale.y;

        transform.translation.y = transform
            .translation
            .y
            .max(window.height() * -0.5 + paddle_height * 0.5)
            .min(window.height() * 0.5 - paddle_height * 0.5);
    }
}

pub fn update_directional_movement(
    mut query: Query<(&Speed, Option<&SpeedFactor>, &Direction, &mut Transform)>,
    time: Res<Time>,
) {
    // Move ball according to direction
    for (speed, speed_factor, direction, mut transform) in query.iter_mut() {
        let actual_speed = if let Some(speed_factor) = speed_factor {
            speed.0 * speed_factor.value
        } else {
            speed.0
        };
        transform.translation += direction.value * actual_speed * time.delta_seconds();
    }
}

pub fn update_ball_collision(
    mut ball_query: Query<(&Ball, &mut Direction, &Transform, &mut SpeedFactor)>,
    inverter_query: Query<(&DirectionInverter, &Transform)>,
    windows: Res<Windows>,
    mut game: ResMut<pong::Game>,
) {
    // Check for collision and adjust direction
    let window = windows.primary();

    for (_ball, mut direction, transform, mut speed_factor) in ball_query.iter_mut() {
        let ball_height: f32 = transform.scale.y;

        // Window collision
        if transform.translation.y < window.height() * -0.5 + ball_height * 0.5
            || transform.translation.y > window.height() * 0.5 - ball_height * 0.5
        {
            direction.value.y *= -1.0;
        }

        // Paddle collision
        for (inverter, inverter_transform) in inverter_query.iter() {
            let collision = bevy::sprite::collide_aabb::collide(
                inverter_transform.translation,
                Vec2::new(inverter_transform.scale.x, inverter_transform.scale.y),
                transform.translation,
                Vec2::new(transform.scale.x, transform.scale.y),
            );

            if !collision.is_none() {
                direction.value.x *= if inverter.axis.x == 1.0 { -1.0 } else { 1.0 };
                direction.value.y *= if inverter.axis.y == 1.0 { -1.0 } else { 1.0 };
                direction.value.z *= if inverter.axis.z == 1.0 { -1.0 } else { 1.0 };

                if inverter.awards_points {
                    game.score += 1;
                }

                speed_factor.value *= 1.1;
            }
        }
    }
}

pub fn update_score(mut query: Query<(&ScoreText, &mut Text)>, game: Res<pong::Game>) {
    let (_score_text, mut text) = query.single_mut();
    text.sections[0].value = format!("Score: {}", game.score);
}

pub fn check_game_over(
    ball_query: Query<(&Ball, &Transform)>,
    player_paddle_query: Query<(&PlayerPaddle, &Transform)>,
    opponent_paddle_query: Query<(&OpponentPaddle, &Transform)>,
    mut state: ResMut<State<pong::GameState>>,
    mut game: ResMut<pong::Game>,
) {
    let (_ball, ball_transform) = ball_query.single();
    let (_ball, player_paddle_transform) = player_paddle_query.single();
    let (_ball, opponent_padde_transform) = opponent_paddle_query.single();

    if ball_transform.translation.x - ball_transform.scale.x * 0.5
        < player_paddle_transform.translation.x
    {
        // player lost
        let _ = state.overwrite_set(pong::GameState::GameOver);
        game.winner = pong::GameWinner::Player2;
    } else if ball_transform.translation.x + ball_transform.scale.x * 0.5
        > opponent_padde_transform.translation.x
    {
        // player won
        let _ = state.overwrite_set(pong::GameState::GameOver);
        game.winner = pong::GameWinner::Player1;
    }
}

#[derive(Component)]
pub struct PlayerPaddle;

#[derive(Component)]
pub struct OpponentPaddle;

#[derive(Component)]
pub struct BorderRestriction {
    pub border_offset: f32,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Direction {
    pub value: Vec3,
}

#[derive(Component)]
pub struct DirectionInverter {
    pub axis: Vec3,
    pub awards_points: bool,
}

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
pub struct SpeedFactor {
    pub value: f32,
}

#[derive(Component)]
pub struct ScoreText;
