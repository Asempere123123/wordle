use yew::prelude::*;

mod backend;
mod components;

use backend::utils::Game;
use components::keyboard::Keyboard;
use components::restart::RestartButton;
use components::words::Words;

#[function_component(App)]
fn app() -> Html {
    let game = use_state(|| Game::new());

    let game_clone = game.clone();
    let onword = Callback::from(move |word: String| {
        let mut game = (&*game_clone).clone();

        game.make_guess(word);
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

    html! {
        <>
            <h1>{ &*game.word }</h1>
            <h1>{ game.won }</h1>

            <Words guesses={game.guesses.clone()} />
            <Keyboard {onword} {onletter} {onremoveletter} guesses={game.guesses.clone()} />
            <RestartButton {onclick} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
