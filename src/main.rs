mod hyperion;
mod interface;
use hyperion::*;
use interface::*;

use macroquad::prelude::*;

#[macroquad::main("Hyperion")]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut init_state = Hyperion::new(0.01, 0.0, 9.1, 1.35, 0.0, 0.5, 0.1);
    let mut states = HyperionStates{states: vec![init_state.clone()]};
    let mut is_saved = false;
    clear_background(BLACK);
    loop {
        match game_state {
            GameState::Menu => {
                init_state = game_state.menu().await;
                states = HyperionStates{states: vec![init_state.clone()]};
            }
            GameState::Animation => {
                game_state.animation(&mut states).await;
            }
            
            GameState::End => {
                game_state.end(&states, &mut is_saved).await;
            }
        }
    }
}
