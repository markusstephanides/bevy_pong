use bevy::prelude::*;

pub mod game;
pub mod game_over;
pub mod pong;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<pong::Game>()
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_state(pong::GameState::Playing)
        .add_plugins(DefaultPlugins)
        .add_startup_system(pong::setup_cameras)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system_set(SystemSet::on_enter(pong::GameState::Playing).with_system(game::setup_game))
        .add_system_set(
            SystemSet::on_update(pong::GameState::Playing)
                .with_system(game::update_player_paddle.label("update_player_paddle"))
                .with_system(
                    game::update_ai_paddles
                        .label("update_ai_paddles")
                        .after("update_player_paddle"),
                )
                .with_system(
                    game::update_paddle_restrictor
                        .label("update_paddle_restrictor")
                        .after("update_ai_paddles"),
                )
                .with_system(
                    game::update_directional_movement
                        .label("directional_movement")
                        .after("update_paddle_restrictor"),
                )
                .with_system(
                    game::update_ball_collision
                        .label("ball_collision")
                        .after("directional_movement"),
                )
                .with_system(game::update_score)
                .with_system(game::check_game_over),
        )
        .add_system_set(SystemSet::on_exit(pong::GameState::Playing).with_system(teardown))
        .add_system_set(
            SystemSet::on_enter(pong::GameState::GameOver).with_system(game_over::setup_game_over),
        )
        .add_system_set(
            SystemSet::on_update(pong::GameState::GameOver)
                .with_system(game_over::gameover_keyboard),
        )
        .add_system_set(SystemSet::on_exit(pong::GameState::GameOver).with_system(teardown))
        .add_system(game_over::gameover_keyboard)
        .run();
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
