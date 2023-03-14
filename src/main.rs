use rusty_engine::prelude::{bevy::utils::tracing::metadata::ParseLevelError, *};

struct GameState {
    health_amount: u8,
    lost: bool,
}

const PLAYER_SPEED: f32 = 250.0;

fn main() {
    let mut game = Game::new();

    // player sprite
    let mut player = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;

    // background music
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // keyboard input
    let mut direction: f32 = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }

    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    let mut player = engine.sprites.get_mut("player1").unwrap();
    player.translation.y = direction * PLAYER_SPEED * engine.delta_f32;
    player.rotation = direction * 0.15;

    if player.translation.y < -360.0 || player.translation.y > 360.0 {
        game_state.health_amount = 0;
    }
}
