use rusty_engine::prelude::*;

struct GameState {
    health_amount: u8,
    lost: bool,
}

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

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {}
