use stylist::yew::styled_component;
use yew::prelude::*;

use crate::backend::utils::Guess;

#[derive(Properties, PartialEq)]
pub struct WordsProps {
    pub guesses: Vec<Guess>,
}

#[styled_component(Words)]
pub fn word(props: &WordsProps) -> Html {
    html! {

        <div class={css!(r#"
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 60px;
            padding-top: 10px;
            box-sizing: border-box;

            .row {
                width: 100%;
                margin-bottom: 10px;
                display: flex;
                justify-content: center;
                gap: 5px;
                max-width: 350px;
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

            @media screen and (max-height: 700px) {
                .tag-container {
                    height: 40px;
                    width: 40px;
                }

                padding: 30px;
                padding-top: 5px;

                .tag {
                    font-weight: lighter;
                }
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
            {props
                .guesses
                .iter()
                .map(|guess| {
                    let left = 5 - guess.guess.len() as i8;

                    html! {
                        <div class="row">
                        {
                            guess
                            .guess
                            .chars()
                            .enumerate()
                            .map(|(i, c)| {
                                html! {
                                    if guess.green.contains(&i) {
                                        <div class="tag-container green">
                                            <span class="tag">{c}</span>
                                        </div>
                                    } else if guess.yellow.contains(&i) {
                                        <div class="tag-container yellow">
                                            <span class="tag">{c}</span>
                                        </div>
                                    } else if guess.red.contains(&i) {
                                        <div class="tag-container red">
                                            <span class="tag">{c}</span>
                                        </div>
                                    } else {
                                        <div class="tag-container">
                                            <span class="tag">{c}</span>
                                        </div>
                                    }
                                }
                            })
                            .collect::<Html>()
                        }
                        {
                            (0..left).map(|_| {
                                html! {
                                <div class="tag-container">
                                    <span class="tag"></span>
                                </div>
                                }
                            }).collect::<Html>()
                        }
                        </div>
                    }
                })
                .collect::<Html>()}
            </div>
    }
}
