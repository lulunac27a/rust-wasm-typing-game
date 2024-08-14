use rand::Rng;
struct GameState {
    score: u32,
    multiplier: f32,
    key_to_type: char,
}
impl GameState {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        GameState {
            score: 0,
            multiplier: 1.0,
            key_to_type: rng.gen_range('a'..='z'),
        }
    }

    fn update_score(&mut self, points: u32) {
        self.score += (points as f32 * self.multiplier) as u32;
    }

    fn set_multiplier(&mut self, new_multiplier: f32) {
        self.multiplier = new_multiplier;
    }

    fn set_key_to_type(&mut self, new_key: char) {
        self.key_to_type = new_key;
    }
}

fn handle_typing(game_state: &mut GameState, typed_key: char) {
    if check_typing(game_state, typed_key) {
        game_state.update_score(100);
        game_state.set_multiplier(game_state.multiplier + 0.01 / game_state.multiplier);
        let mut rng = rand::thread_rng();
        game_state.set_key_to_type(rng.gen_range('a'..='z'));
    } else {
        game_state.set_multiplier(game_state.multiplier * 0.99);
    }
}

fn check_typing(game_state: &mut GameState, key: char) -> bool {
    if key == game_state.key_to_type {
        true
    } else {
        false
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]

pub struct Game {
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::new(),
        }
    }
    pub fn handle_typing(&mut self, typed_key: char) {
        handle_typing(&mut self.state, typed_key);
    }
    pub fn get_score(&self) -> u32 {
        self.state.score
    }
    pub fn get_multiplier(&self) -> f32 {
        self.state.multiplier
    }
    pub fn get_key_to_type(&self) -> char {
        self.state.key_to_type
    }
}
