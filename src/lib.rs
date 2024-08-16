use rand::Rng;
use wasm_bindgen::prelude::*;

struct GameState {
    score: u32,                //game score
    multiplier: f32,           //score multiplier
    character_to_type: String, //character to type
}
impl GameState {
    fn new() -> Self {
        //create new game object
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng(); //random object
        GameState {
            score: 0,                                                //set score to 0
            multiplier: 1.0,                                         //set multiplier to 1
            character_to_type: rng.gen_range('a'..='z').to_string(), //set character to type to random character from character set
        }
    }

    fn update_score(&mut self, points: u32) {
        //update score
        self.score += (points as f32 * self.multiplier) as u32; //increase score based on amount and multiplier
    }

    fn set_multiplier(&mut self, new_multiplier: f32) {
        //update score multiplier
        self.multiplier = new_multiplier; //set score multiplier value
    }

    fn set_character_to_type(&mut self, new_key: String) {
        //update character to type
        self.character_to_type = new_key.to_string(); //set character to type
    }
}

fn handle_typing(game_state: &mut GameState, typed_key: String) {
    //handle typing input
    if check_typing(game_state, typed_key) {
        //check if entered text is correct (entered text is equal to character to type)
        game_state.update_score(100); //increase points
        game_state.set_multiplier(game_state.multiplier + 0.01 / game_state.multiplier.max(1.0)); //increase score multiplier
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng(); //random object
        game_state.set_character_to_type(rng.gen_range('a'..='z').to_string()); //set character to type to random character from character list
    } else {
        game_state.set_multiplier(game_state.multiplier * 0.99); //decrease score multiplier
    }
}

fn check_typing(game_state: &mut GameState, key: String) -> bool {
    //check if entered text is equal to character to type
    if key == game_state.character_to_type.to_string() {
        //if correct
        true
    } else {
        //if incorrect
        false
    }
}

#[wasm_bindgen]

pub struct Game {
    state: GameState, //game state
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            state: GameState::new(), //initialize new game
        }
    }
    pub fn check_typing(&mut self, key: String) {
        //check if entered text is correct (character to type is equal to entered text)
        check_typing(&mut self.state, key);
    }
    pub fn handle_typing(&mut self, typed_key: String) {
        //handle typing input
        handle_typing(&mut self.state, typed_key);
    }
    pub fn get_score(&self) -> u32 {
        //get current score
        self.state.score
    }
    pub fn get_multiplier(&self) -> f32 {
        //get current score multiplier
        self.state.multiplier
    }
    pub fn get_character_to_type(&self) -> String {
        //get current character to type
        self.state.character_to_type.to_string()
    }
}
