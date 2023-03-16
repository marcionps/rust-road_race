use rand::prelude::*;
use rusty_engine::prelude::*;

struct GameState {
    health_amount0: u8,
    health_amount1: u8,
    lost: bool,
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
const INITIAL_HEALTH: u8 = 5;

fn main() {
    let mut game = Game::new();
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];

    // player sprite
    let mut player0 = game.add_sprite("player0", SpritePreset::RacingCarBlue);
    player0.translation.x = -500.0;
    player0.translation.y = -100.0;
    player0.layer = 10.0;
    player0.collision = true;

    let mut player1 = game.add_sprite("player1", SpritePreset::RacingCarRed);
    player1.translation.x = -500.0;
    player1.translation.y = 100.0;
    player1.layer = 10.0;
    player1.collision = true;

    // road lines
    for i in 0..10 {
        let mut roadline =
            game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    // obstacles
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // health text
    let health_message0 =
        game.add_text("health_message1", format!("Health P1: {}", INITIAL_HEALTH));
    health_message0.translation = Vec2::new(550.0, 320.0);

    let health_message1 =
        game.add_text("health_message2", format!("Health P2: {}", INITIAL_HEALTH));
    health_message1.translation = Vec2::new(550.0, 280.0);

    // background music
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount0: INITIAL_HEALTH,
        health_amount1: INITIAL_HEALTH,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost {
        return;
    }

    // keyboard input
    let mut direction0: f32 = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction0 += 1.0;
    }

    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction0 -= 1.0;
    }

    let mut direction1: f32 = 0.0;
    if engine.keyboard_state.pressed(KeyCode::W) {
        direction1 += 1.0;
    }

    if engine.keyboard_state.pressed(KeyCode::S) {
        direction1 -= 1.0;
    }

    let mut players = engine.sprites.get_many_mut(["player0", "player1"]).unwrap();

    players[0].translation.y += direction0 * PLAYER_SPEED * engine.delta_f32;
    players[0].rotation = direction0 * 0.15;

    players[1].translation.y += direction1 * PLAYER_SPEED * engine.delta_f32;
    players[1].rotation = direction1 * 0.15;

    if players[0].translation.y < -360.0 || players[0].translation.y > 360.0 {
        game_state.health_amount0 = 0;
    }

    if players[1].translation.y < -360.0 || players[1].translation.y > 360.0 {
        game_state.health_amount1 = 0;
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.00;
            }
        }

        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    let mut health_message = engine
        .texts
        .get_many_mut(["health_message1", "health_message2"])
        .unwrap();

    for event in engine.collision_events.drain(..) {
        if !(event.pair.either_contains("player0") || event.pair.either_contains("player1"))
            || (event.pair.either_contains("player0") && event.pair.either_contains("player1"))
            || event.state.is_end()
        {
            continue;
        }

        println!("{:?}", event);

        if game_state.health_amount0 > 0 && event.pair.either_contains("player0") {
            game_state.health_amount0 -= 1;

            health_message[0].value = format!("Health P1: {}", game_state.health_amount0);
        }

        if game_state.health_amount1 > 0 && event.pair.either_contains("player1") {
            game_state.health_amount1 -= 1;
            health_message[1].value = format!("Health P2: {}", game_state.health_amount1);
        }
        engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
    }

    if game_state.health_amount0 == 0 || game_state.health_amount1 == 0 {
        game_state.lost = true;

        let game_over = engine.add_text("game_over", "Game Over");
        game_over.font_size = 128.0;
        game_over.translation.y = 100.0;

        let win = engine.add_text("win", "");

        if game_state.health_amount0 == 0 {
            win.value = "Player 2 WINS!".to_string();
        } else {
            win.value = "Player 1 WINS!".to_string();
        }

        win.font_size = 120.0;
        win.translation.y = -100.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}
