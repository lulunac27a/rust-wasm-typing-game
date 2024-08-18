//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use rust_wasm_typing_game::Game;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn initialize_game() {
    //initialize game
    let game = Game::new(); //create new game object
    assert_eq!(60, game.get_time_remaining());
    assert_eq!(1, game.get_multiplier());
    assert_eq!(1, game.get_difficulty());
    assert_eq!(0, game.get_score());
}

#[wasm_bindgen_test]
fn test_points() {
    //test for points earned with score multiplier
    let mut game = Game::new();
    game.update_score(500);
    game.set_multiplier(2.5);
    game.update_score(100);
    assert_eq!(750, game.get_score());
}

#[wasm_bindgen_test]
fn test_correct_typing() {
    //test for correct key pressed
    let mut game = Game::new();
    game.handle_typing(game.get_character_to_type());
    assert_eq!(120, game.get_score());
    assert!(game.get_multiplier() > 1.02);
}

#[wasm_bindgen_test]
fn test_incorrect_typing() {
    //test for wrong key pressed
    let mut game = Game::new();
    game.handle_typing("invalid".to_string());
    assert_eq!(0, game.get_score());
    assert_eq!(0.99, game.get_multiplier());
}
