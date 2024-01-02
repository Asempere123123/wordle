use yew::prelude::*;

use crate::backend::utils::Guess;

#[derive(Properties, PartialEq)]
pub struct KeyboardProps {
    // Podria ser un callback de keyboardaction donde keyboardaction es un enum
    pub onword: Callback<String>,
    pub onletter: Callback<char>,
    pub onremoveletter: Callback<()>,
    pub guesses: Vec<Guess>,
}

#[function_component(Keyboard)]
pub fn keyboard(props: &KeyboardProps) -> Html {
    let mut green = Vec::new();
    let mut yellow = Vec::new();
    let mut red = Vec::new();
    props.guesses.iter().for_each(|guess| {
        guess.guess.chars().enumerate().for_each(|(i, c)| {
            if guess.green.contains(&i) {
                green.push(c);
            } else if guess.yellow.contains(&i) {
                yellow.push(c);
            } else if guess.red.contains(&i) {
                red.push(c);
            }
        })
    });

    let first_line = vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '⇦'];
    let second_line = vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
    let third_line = vec!['z', 'x', 'c', 'v', 'b', 'n', 'm', '⏎'];

    let word = use_state(|| "".to_owned());
    let onword = props.onword.clone();
    let onletter = props.onletter.clone();
    let onremoveletter = props.onremoveletter.clone();
    let onclick = Callback::from(move |c| {
        let mut word_content = (&*word).clone();

        if c == '⏎' {
            if word_content.len() == 5 {
                onword.emit(word_content);
                word.set("".to_owned());
            }
            return;
        }
        if c == '⇦' {
            word_content = match word_content.char_indices().next_back() {
                Some((i, _)) => {
                    onremoveletter.emit(());
                    word_content[..i].to_owned()
                }
                None => word_content,
            }
        } else {
            if word_content.len() < 5 {
                onletter.emit(c);
                word_content.push(c);
            }
        }

        word.set(word_content);
    });

    html! {
    <>
        <KeyLine line={first_line} onclick={onclick.clone()} green={green.clone()} yellow={yellow.clone()} red={red.clone()} />
        <KeyLine line={second_line} onclick={onclick.clone()} green={green.clone()} yellow={yellow.clone()} red={red.clone()} />
        <KeyLine line={third_line} {onclick} green={green} yellow={yellow} red={red} />
    </>
    }
}

#[derive(Properties, PartialEq)]
struct KeyLineProps {
    line: Vec<char>,
    green: Vec<char>,
    yellow: Vec<char>,
    red: Vec<char>,
    onclick: Callback<char>,
}

#[function_component(KeyLine)]
fn key_line(props: &KeyLineProps) -> Html {
    html! {
        {for props.line.iter().map(|c| {
            let onclick_clone = props.onclick.clone();
            let c_clone = c.clone();
            let onclick = Callback::from(move |_| {
                onclick_clone.emit(c_clone);
            });

            html! {
                if props.green.contains(c) {
                <button {onclick} style="background-color: green;">{c}</button>}
                else if props.yellow.contains(c) {
                <button {onclick} style="background-color: yellow;">{c}</button>}
                else if props.red.contains(c) {
                    <button {onclick} style="background-color: red;">{c}</button>
                } else {
                <button {onclick}>{c}</button>}
            }
        })}
    }
}
