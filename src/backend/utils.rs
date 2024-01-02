use rand::Rng;

const WORD_LIST: &str = include_str!("words.txt");

#[derive(Clone, Debug)]
pub struct Game {
    pub word: String,
    pub guesses: Vec<Guess>,
    pub won: bool,
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
        }
    }

    pub fn make_guess(self: &mut Self, guess: String) {
        let word_chars: Vec<char> = self.word.chars().collect();

        let mut green = Vec::new();
        let mut yellow = Vec::new();
        let mut red = Vec::new();
        guess.chars().enumerate().for_each(|(i, c)| {
            if c == word_chars[i] {
                green.push(i);
            } else if word_chars.contains(&c) {
                yellow.push(i);
            } else {
                red.push(i);
            }
        });

        if green.len() == 5 {
            self.won = true;
        }

        let guess = Guess {
            guess,
            green,
            yellow,
            red,
        };

        self.guesses[self.current_word] = guess;

        self.current_word += 1;
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
