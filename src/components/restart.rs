use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RestartButtonProps {
    pub onclick: Callback<()>,
}

#[function_component(RestartButton)]
pub fn restart(props: &RestartButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let restart_game = Callback::from(move |_| onclick.emit(()));

    html! {
        <button onclick={restart_game} class="btn btn-primary">{"Restart"}</button>
    }
}
