use rand::prelude::SliceRandom;
use rand::Rng;
use wasm_bindgen::prelude::*;

struct GameState {
    score: u32,                //game score
    multiplier: f32,           //score multiplier
    difficuity: u32,           //game difficulty
    character_to_type: String, //character to type
}
impl GameState {
    fn new() -> Self {
        //create new game object
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng(); //random object
        GameState {
            score: 0,                                                //set score to 0
            multiplier: 1.0,                                         //set multiplier to 1
            difficuity: 1,                                           //set difficulty to 1
            character_to_type: rng.gen_range('a'..='z').to_string(), //set character to type to random character from character set
        }
    }

    fn update_score(&mut self, points: u32) {
        //update score
        self.score += (points as f32 * self.multiplier * self.difficuity as f32).floor() as u32;
        //increase score based on amount and multiplier
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
        let multiplier_bonus: f32 = if game_state.multiplier < 1.0 {
            0.02 - game_state.multiplier * 0.01
        } else {
            0.01 / game_state.multiplier.max(1.0)
        }; //increase score multiplier based on currect score multiplier
        game_state.set_multiplier(game_state.multiplier + multiplier_bonus); //increase score multiplier
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng(); //random object
        let lowercase_characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); //lowercase characters
        let uppercase_characters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); //uppercase characters
        let numeric_digit_characters: Vec<char> = "0123456789".chars().collect(); //numeric digits
        let symbol_characters: Vec<char> = "~!@#$%^&*()_+`-=[]\\;',./{}|:\"<>?".chars().collect(); //special characters (symbols)
        if game_state.difficuity == 1 {
            game_state
                .set_character_to_type(lowercase_characters.choose(&mut rng).unwrap().to_string());
        //set character to type to random lowercase character from character list
        } else if game_state.difficuity == 2 {
            let rand_value: f32 = rng.gen();
            if rand_value < 1.0 / 2.0 {
                game_state.set_character_to_type(
                    lowercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random lowercase character from character list
            } else {
                game_state.set_character_to_type(
                    uppercase_characters.choose(&mut rng).unwrap().to_string(),
                )
            }; //set character to type to random uppercase character from character list
        } else if game_state.difficuity == 3 {
            let rand_value: f32 = rng.gen();
            if rand_value < 26.0 / 62.0 {
                game_state.set_character_to_type(
                    lowercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random lowercase character from character list
            }
            if rand_value < 52.0 / 62.0 {
                game_state.set_character_to_type(
                    uppercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random uppercase character from character list
            } else {
                game_state.set_character_to_type(
                    numeric_digit_characters
                        .choose(&mut rng)
                        .unwrap()
                        .to_string(),
                )
            }; //set character to type to random numeric digit character from character list
        } else if game_state.difficuity == 4 {
            let rand_value: f32 = rng.gen();
            if rand_value < 26.0 / 94.0 {
                game_state.set_character_to_type(
                    lowercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random lowercase character from character list
            }
            if rand_value < 52.0 / 94.0 {
                game_state.set_character_to_type(
                    uppercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random uppercase character from character list
            }
            if rand_value < 62.0 / 94.0 {
                game_state.set_character_to_type(
                    numeric_digit_characters
                        .choose(&mut rng)
                        .unwrap()
                        .to_string(),
                ); //set character to type to random numeric digit character from character list
            } else {
                game_state
                    .set_character_to_type(symbol_characters.choose(&mut rng).unwrap().to_string())
            }; //set character to type to random symbol character from character list
        }
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
