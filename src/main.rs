use bevy::prelude::*;

const PADDLE_HEIGHT_FACTOR: f32 = 0.2;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameWinner {
    None,
    Player1,
    Player2,
}

#[derive(Component)]
struct PlayerPaddle;

#[derive(Component)]
struct OpponentPaddle;

#[derive(Component)]
struct BorderRestriction {
    pub border_offset: f32,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Direction {
    pub value: Vec3,
}

#[derive(Component)]
struct DirectionInverter {
    pub axis: Vec3,
    pub awards_points: bool,
}

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct SpeedFactor {
    pub value: f32,
}

#[derive(Default)]
struct Game {
    pub score: i32,
    pub winner: GameWinner,
}

impl Default for GameWinner {
    fn default() -> Self {
        return GameWinner::None
    }
}

#[derive(Component)]
struct ScoreText;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<Game>()
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_state(GameState::Playing)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_cameras)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_game))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_player_paddle)
                .with_system(update_opponent_paddle)
                .with_system(update_paddle_restrictor)
                .with_system(update_directional_movement.label("directional_movement"))
                .with_system(
                    update_ball_collision
                        .label("ball_collision")
                        .after("directional_movement"),
                )
                .with_system(update_score)
                .with_system(check_game_over),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown))
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(setup_game_over))
        .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(gameover_keyboard))
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown))
        .add_system(gameover_keyboard)
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_game(mut commands: Commands, windows: Res<Windows>, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    let window = windows.primary();

    game.winner = GameWinner::None;
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

fn setup_game_over(mut commands: Commands, windows: Res<Windows>, asset_server: Res<AssetServer>, game: Res<Game>) {
    let window = windows.primary();
   
    // ui
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                if game.winner == GameWinner::Player1 { "You win!" } else { "You lose!" },
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
        })
        .insert(ScoreText);
}

fn update_player_paddle(
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

fn update_opponent_paddle(
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

fn update_paddle_restrictor(
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

fn update_directional_movement(
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

fn update_ball_collision(
    mut ball_query: Query<(&Ball, &mut Direction, &Transform, &mut SpeedFactor)>,
    inverter_query: Query<(&DirectionInverter, &Transform)>,
    windows: Res<Windows>,
    mut game: ResMut<Game>,
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

fn update_score(mut query: Query<(&ScoreText, &mut Text)>, game: Res<Game>) {
    let (_score_text, mut text) = query.single_mut();
    text.sections[0].value = format!("Score: {}", game.score);
}

fn check_game_over(
    ball_query: Query<(&Ball, &Transform)>,
    player_paddle_query: Query<(&PlayerPaddle, &Transform)>,
    opponent_paddle_query: Query<(&OpponentPaddle, &Transform)>,
    mut state: ResMut<State<GameState>>,
    mut game: ResMut<Game>,
) {
    let (_ball, ball_transform) = ball_query.single();
    let (_ball, player_paddle_transform) = player_paddle_query.single();
    let (_ball, opponent_padde_transform) = opponent_paddle_query.single();

    if ball_transform.translation.x - ball_transform.scale.x * 0.5
        < player_paddle_transform.translation.x
    {
        // player lost
        let _ = state.overwrite_set(GameState::GameOver);
        game.winner = GameWinner::Player2;
    } else if ball_transform.translation.x + ball_transform.scale.x * 0.5
        > opponent_padde_transform.translation.x
    {
        // player won
        let _ = state.overwrite_set(GameState::GameOver);
        game.winner = GameWinner::Player1;
    }
}

fn gameover_keyboard(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let _ = state.overwrite_set(GameState::Playing);
    }
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}