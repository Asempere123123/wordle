use std::collections::HashMap;

use rand::Rng;

const WORD_LIST: &str = include_str!("words.txt");
const ALL_WORD_LIST: &str = include_str!("all_words.txt");

#[derive(Clone, Debug)]
pub struct Game {
    pub word: String,
    pub guesses: Vec<Guess>,
    pub won: bool,
    pub game_ended: bool,
    pub current_word: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Guess {
    pub guess: String,
    pub green: Vec<usize>,
    pub yellow: Vec<usize>,
    pub red: Vec<usize>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            word: Self::get_random_word(),
            guesses: vec![Guess::new(); 6],
            won: false,
            current_word: 0,
            game_ended: false,
        }
    }

    pub fn make_guess(self: &mut Self, guess: String) -> Option<String> {
        let all_words: Vec<&str> = ALL_WORD_LIST.lines().collect();

        if !all_words.contains(&guess.as_str()) {
            self.guesses[self.current_word] = Guess::new();
            return Some("La palabra no existe".to_owned());
        }

        let word_chars: Vec<char> = self.word.chars().collect();

        let mut green = Vec::new();
        let mut yellow = Vec::new();
        let mut red = Vec::new();
        let mut coincidences: HashMap<char, usize> = HashMap::new();
        guess.chars().enumerate().for_each(|(i, c)| {
            if c == word_chars[i] {
                green.push(i);
                if let Some(coincidence) = coincidences.get_mut(&c) {
                    *coincidence += 1;
                } else {
                    coincidences.insert(c, 1);   
                }
            }
        });

        let mut appearences: HashMap<char, usize> = HashMap::new();
        guess.chars().enumerate().for_each(|(i, c)| {
            if let None = appearences.get(&c) {
                let count = self.word.chars().filter(|d| d == &c).count();
                appearences.insert(c, count);
            }
            let coin = coincidences.get(&c).unwrap_or(&0);
            let ap = appearences.get(&c).unwrap();
            if word_chars.contains(&c) && !(c == word_chars[i]) && ap > coin {
                yellow.push(i);
                if let Some(coin) = coincidences.get_mut(&c) {
                    *coin += 1;
                } else {
                    coincidences.insert(c, 1);
                }
            } else {
                red.push(i);
            }
        });

        if green.len() == 5 {
            self.won = true;
            self.game_ended = true;
        } else if self.current_word == 5 {
            self.game_ended = true;
        }

        let guess = Guess {
            guess,
            green,
            yellow,
            red,
        };

        self.guesses[self.current_word] = guess;

        self.current_word += 1;
        None
    }

    fn get_random_word() -> String {
        let words: Vec<&str> = WORD_LIST.lines().collect();

        let random_index = rand::thread_rng().gen_range(0..words.len());
        words[random_index].to_string()
    }

    pub fn write_letter(self: &mut Self, letter: char) {
        let current_guess = &mut self.guesses[self.current_word];

        current_guess.guess.push(letter);
    }

    pub fn delete_letter(self: &mut Self) {
        let current_guess = &mut self.guesses[self.current_word];

        current_guess.guess.pop();
    }
}

impl Guess {
    fn new() -> Self {
        Guess {
            guess: "".to_owned(),
            green: Vec::new(),
            yellow: Vec::new(),
            red: Vec::new(),
        }
    }
}
