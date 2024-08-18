use rand::prelude::SliceRandom;
use rand::Rng;
use wasm_bindgen::prelude::*;

struct GameState {
    score: u32,                //game score
    multiplier: f32,           //score multiplier
    difficulty: u32,           //game difficulty
    character_to_type: String, //character to type
    time_limit: u32,           //time remaining in seconds
    time_remaining: u32,       //time remaining in seconds
}
impl GameState {
    fn new() -> Self {
        //create new game object
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng(); //random object
        GameState {
            score: 0,                                                //set score to 0
            multiplier: 1.0,                                         //set multiplier to 1
            difficulty: 1,                                           //set difficulty to 1
            character_to_type: rng.gen_range('a'..='z').to_string(), //set character to type to random character from character set
            time_limit: 60,                                          //set time limit to 60 seconds
            time_remaining: 60, //set time remaining to 60 seconds
        }
    }

    fn update_score(&mut self, points: u32) {
        //update score
        self.score += (points as f32 * self.multiplier * self.difficulty as f32).floor() as u32;
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
        game_state.update_score(game_state.time_limit + game_state.time_remaining); //increase points based on time remaining and time limit
        let multiplier_bonus: f32 = if game_state.multiplier < 1.0 {
            0.02 - game_state.multiplier * 0.01
        } else {
            0.01 / game_state.multiplier.max(1.0)
        }; //increase score multiplier based on currect score multiplier
        let time_bonus: f32 = 1.0
            / (game_state.time_limit as f32 - game_state.time_remaining as f32
                + game_state.time_limit as f32) as f32; //increase score multiplier based on time limit and time remaining
        game_state.set_multiplier(game_state.multiplier + multiplier_bonus + time_bonus); //increase score multiplier
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng(); //random object
        let lowercase_characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); //lowercase characters
        let uppercase_characters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); //uppercase characters
        let numeric_digit_characters: Vec<char> = "0123456789".chars().collect(); //numeric digits
        let symbol_characters: Vec<char> = "~!@#$%^&*()_+`-=[]\\;',./{}|:\"<>?".chars().collect(); //special characters (symbols)
        if game_state.difficulty == 1 {
            game_state
                .set_character_to_type(lowercase_characters.choose(&mut rng).unwrap().to_string());
        //set character to type to random lowercase character from character list
        } else if game_state.difficulty == 2 {
            let rand_value: f32 = rng.gen();
            if rand_value < (1.0 / 2.0) {
                game_state.set_character_to_type(
                    lowercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random lowercase character from character list
            } else {
                game_state.set_character_to_type(
                    uppercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random uppercase character from character list
            }
        } else if game_state.difficulty == 3 {
            let rand_value: f32 = rng.gen();
            if rand_value < (26.0 / 62.0) {
                game_state.set_character_to_type(
                    lowercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random lowercase character from character list
            } else if rand_value < (52.0 / 62.0) {
                game_state.set_character_to_type(
                    uppercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random uppercase character from character list
            } else {
                game_state.set_character_to_type(
                    numeric_digit_characters
                        .choose(&mut rng)
                        .unwrap()
                        .to_string(),
                ); //set character to type to random numeric digit character from character list
            }
        } else if game_state.difficulty == 4 {
            let rand_value: f32 = rng.gen();
            if rand_value < (26.0 / 94.0) {
                game_state.set_character_to_type(
                    lowercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random lowercase character from character list
            } else if rand_value < (52.0 / 94.0) {
                game_state.set_character_to_type(
                    uppercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random uppercase character from character list
            } else if rand_value < (62.0 / 94.0) {
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
    pub fn get_difficulty(&self) -> u32 {
        //get current difficulty
        self.state.difficulty
    }
    pub fn get_character_to_type(&self) -> String {
        //get current character to type
        self.state.character_to_type.to_string()
    }
    pub fn get_time_remaining(&self) -> u32 {
        //get current time remaining in seconds
        self.state.time_remaining
    }
    pub fn set_difficulty(&mut self, difficulty: u32) {
        //set current difficulty
        self.state.difficulty = difficulty; //set difficulty
    }
    pub fn set_time_limit(&mut self, time: u32) {
        //set time limit in seconds
        self.state.time_limit = time; //set time limit in seconds
    }
    pub fn set_time_remaining(&mut self, time: u32) {
        //set time remaining in seconds
        self.state.time_remaining = time; //set time remaining in seconds
    }
    pub fn decrease_time(&mut self) {
        //decrease time remaining
        if self.state.time_remaining > 0 {
            //check if time remaining is greater than 0
            self.state.time_remaining -= 1; //decrease time remaining by 1
        }
    }
    pub fn update_score(&mut self, points: u32) {
        //update score
        self.state.score +=
            (points as f32 * self.state.multiplier * self.state.difficulty as f32).floor() as u32;
        //increase score based on amount and multiplier
    }

    pub fn set_multiplier(&mut self, new_multiplier: f32) {
        //update score multiplier
        self.state.multiplier = new_multiplier; //set score multiplier value
    }

    pub fn set_character_to_type(&mut self, new_key: String) {
        //update character to type
        self.state.character_to_type = new_key.to_string(); //set character to type
    }
    pub fn generate_character_to_type(&mut self) {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng(); //random object
        let lowercase_characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); //lowercase characters
        let uppercase_characters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); //uppercase characters
        let numeric_digit_characters: Vec<char> = "0123456789".chars().collect(); //numeric digits
        let symbol_characters: Vec<char> = "~!@#$%^&*()_+`-=[]\\;',./{}|:\"<>?".chars().collect(); //special characters (symbols)
        if self.state.difficulty == 1 {
            self.state
                .set_character_to_type(lowercase_characters.choose(&mut rng).unwrap().to_string());
        //set character to type to random lowercase character from character list
        } else if self.state.difficulty == 2 {
            let rand_value: f32 = rng.gen();
            if rand_value < (1.0 / 2.0) {
                self.state.set_character_to_type(
                    lowercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random lowercase character from character list
            } else {
                self.state.set_character_to_type(
                    uppercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random uppercase character from character list
            }
        } else if self.state.difficulty == 3 {
            let rand_value: f32 = rng.gen();
            if rand_value < (26.0 / 62.0) {
                self.state.set_character_to_type(
                    lowercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random lowercase character from character list
            } else if rand_value < (52.0 / 62.0) {
                self.state.set_character_to_type(
                    uppercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random uppercase character from character list
            } else {
                self.state.set_character_to_type(
                    numeric_digit_characters
                        .choose(&mut rng)
                        .unwrap()
                        .to_string(),
                ); //set character to type to random numeric digit character from character list
            }
        } else if self.state.difficulty == 4 {
            let rand_value: f32 = rng.gen();
            if rand_value < (26.0 / 94.0) {
                self.state.set_character_to_type(
                    lowercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random lowercase character from character list
            } else if rand_value < (52.0 / 94.0) {
                self.state.set_character_to_type(
                    uppercase_characters.choose(&mut rng).unwrap().to_string(),
                ); //set character to type to random uppercase character from character list
            } else if rand_value < (62.0 / 94.0) {
                self.state.set_character_to_type(
                    numeric_digit_characters
                        .choose(&mut rng)
                        .unwrap()
                        .to_string(),
                ); //set character to type to random numeric digit character from character list
            } else {
                self.state
                    .set_character_to_type(symbol_characters.choose(&mut rng).unwrap().to_string())
            }; //set character to type to random symbol character from character list
        }
    }
}
