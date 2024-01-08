use stylist::yew::styled_component;
use yew::prelude::*;
use yew_bootstrap::component::Container;
use yew_bootstrap::util::{include_cdn, include_cdn_js};

mod backend;
mod components;

use backend::utils::Game;
use components::finishscreen::FinishScreen;
use components::keyboard::Keyboard;
use components::restart::RestartButton;
use components::words::Words;

#[styled_component(App)]
fn app() -> Html {
    let game = use_state(|| Game::new());
    let alerts: UseStateHandle<Vec<String>> = use_state(|| Vec::new());

    let game_clone = game.clone();
    let alerts_clone = alerts.clone();
    let onword = Callback::from(move |word: String| {
        let mut game = (&*game_clone).clone();

        match game.make_guess(word) {
            None => {}
            Some(error) => {
                let mut alerts = (*alerts_clone).clone();
                alerts.push(error);
                alerts_clone.set(alerts);
            }
        }
        game_clone.set(game);
    });

    let game_clone = game.clone();
    let onletter = Callback::from(move |c| {
        let mut game = (&*game_clone).clone();

        game.write_letter(c);
        game_clone.set(game);
    });

    let game_clone = game.clone();
    let onremoveletter = Callback::from(move |_| {
        let mut game = (&*game_clone).clone();

        game.delete_letter();
        game_clone.set(game);
    });

    let game_clone = game.clone();
    let onclick = Callback::from(move |_| game_clone.set(Game::new()));

    let game_clone = game.clone();
    let onclose = Callback::from(move |_| {
        let mut game = (&*game_clone).clone();

        game.game_ended = false;
        game_clone.set(game);
    });

    html! {
        <>
            {include_cdn()}
            <Container>
                <div class={css!(r#"
                    width: 100%;
                    display: flex;
                    justify-content: center;
                    padding-top: 40px;
                "#)}>
                    <h1>{ "Asemperedle" }</h1>
                </div>
                <div class={css!(r#"
                    position: absolute;
                    right: 0;
                    top: 0;
                    padding: 10px;
                    display: grid;

                    .btn {
                        justify-self: end;
                        margin-bottom:  5px;
                    }
                "#)}>
                    <RestartButton onclick={onclick.clone()} />
                    {alerts.iter().enumerate().map(|(index, alert)| {
                        let alerts_clone = alerts.clone();
                        let alert_onclose = {
                            Callback::from(move |_| {
                                let alerts = (*alerts_clone).clone();
                                let mut alerts = alerts.clone();
                                alerts.remove(index);
                                alerts_clone.set(alerts);
                            })
                            };
                        html! {
                            <div class="alert alert-warning alert-dismissible fade show" role="alert">
                                {alert}
                                <button type="button" class="btn-close" aria-label="Close" onclick={alert_onclose}></button>
                            </div>
                        }
                    }).collect::<Html>()}
                </div>
                <Words guesses={game.guesses.clone()} />
                <Keyboard {onword} {onletter} {onremoveletter} guesses={game.guesses.clone()} is_finish={game.won} />

                if game.game_ended {
                    <FinishScreen is_win={game.won} {onclose} onrestart={onclick} />
                }
            </Container>
            <h1>{&*game.word}</h1>
            {include_cdn_js()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
