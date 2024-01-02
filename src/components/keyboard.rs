use stylist::yew::styled_component;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::backend::utils::Guess;

#[derive(Properties, PartialEq)]
pub struct KeyboardProps {
    // Podria ser un callback de keyboardaction donde keyboardaction es un enum
    pub onword: Callback<String>,
    pub onletter: Callback<char>,
    pub onremoveletter: Callback<()>,
    pub guesses: Vec<Guess>,
}

#[styled_component(Keyboard)]
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

    let first_line = vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
    let second_line = vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
    let third_line = vec!['⏎', 'z', 'x', 'c', 'v', 'b', 'n', 'm', '⇦'];

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

    let onclick_clone = onclick.clone();
    use_event_with_window("keydown", move |e: KeyboardEvent| {
        unsafe {
            let mut ch = char::from_u32_unchecked(e.key_code());

            if ch == 13.into() {
                ch = '⏎';
            } else if ch == 8.into() {
                ch = '⇦';
            } else {
                ch = ch.into();
            }
            ch = ch.to_ascii_lowercase();

            let all_keys = vec![
                'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '⇦', 'a', 's', 'd', 'f', 'g', 'h',
                'j', 'k', 'l', 'z', 'x', 'c', 'v', 'b', 'n', 'm', '⏎',
            ];
            if all_keys.contains(&ch) {
                onclick_clone.emit(ch);
            }
        }
    });

    html! {
    <div class={css!(r#"
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
            align-items: center;
            box-sizing: border-box;

            .row {
                width: 100%;
                margin-bottom: 10px;
                display: flex;
                justify-content: center;
                max-width: 750px;
                gap: 5px;
            }

            .tag-container {
                display: flex;
                justify-content: center;
                align-items: center;
                height: 60px;
                width: 60px;
                border: 1px solid black;
                border-radius: 7px;
            }

            .tag {
                font-size: 32px;
            }

            .green {
                background-color: #43a047;
            }

            .yellow {
                background-color: #e4a81d;
            }

            .red {
                background-color: #757575;
            }
        "#)}>
        <KeyLine line={first_line} onclick={onclick.clone()} green={green.clone()} yellow={yellow.clone()} red={red.clone()} />
        <KeyLine line={second_line} onclick={onclick.clone()} green={green.clone()} yellow={yellow.clone()} red={red.clone()} />
        <KeyLine line={third_line} {onclick} green={green} yellow={yellow} red={red} />
    </div>
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
        <div class="row">
            {props.line.iter().map(|c| {
                let onclick_clone = props.onclick.clone();
                let c_clone = c.clone();
                let onclick = Callback::from(move |_| {
                    onclick_clone.emit(c_clone);
                });

                html! {
                    if props.green.contains(c) {
                    <button {onclick} class="tag-container green">
                        <span class="tag">{c}</span>
                    </button>
                    } else if props.yellow.contains(c) {
                        <button {onclick} class="tag-container yellow">
                            <span class="tag">{c}</span>
                        </button>
                    } else if props.red.contains(c) {
                    <button {onclick} class="tag-container red">
                        <span class="tag">{c}</span>
                    </button>
                    } else {
                    <button {onclick} class="tag-container">
                        <span class="tag">{c}</span>
                    </button>
                    }
                }
            }).collect::<Html>()}
        </div>
    }
}
