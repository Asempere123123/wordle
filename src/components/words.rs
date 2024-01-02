use yew::prelude::*;

use crate::backend::utils::Guess;

#[derive(Properties, PartialEq)]
pub struct WordsProps {
    pub guesses: Vec<Guess>,
}

#[function_component(Words)]
pub fn word(props: &WordsProps) -> Html {
    props
        .guesses
        .iter()
        .map(|guess| {
            guess
                .guess
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    html! {
                        if guess.green.contains(&i) {
                            <div class="letter" style="color: green;">{c}</div>
                        } else if guess.yellow.contains(&i) {
                            <div class="letter" style="color: yellow;">{c}</div>
                        } else if guess.red.contains(&i) {
                            <div class="letter" style="color: red;">{c}</div>
                        } else {
                            <div class="letter">{c}</div>
                        }
                    }
                })
                .collect::<Html>()
        })
        .collect()
}
