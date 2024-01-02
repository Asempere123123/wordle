use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FinishScreenProps {
    pub is_win: bool,
    pub onrestart: Callback<()>,
    pub onclose: Callback<()>,
}

#[styled_component(FinishScreen)]
pub fn finishscreen(props: &FinishScreenProps) -> Html {
    let onclose_clone = props.onclose.clone();
    let onclose = Callback::from(move |_| {
        onclose_clone.emit(());
    });

    let onrestart_clone = props.onrestart.clone();
    let onrestart = Callback::from(move |_| {
        onrestart_clone.emit(());
    });

    html! {
        <div class="modal" tabindex="-1" style="display: block;">
            <div class="modal-dialog">
                <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title">
                        if props.is_win {
                            {"Has Ganado"}
                        } else {
                            {"Has Perdido"}
                        }
                    </h5>
                    <button onclick={onclose} type="button" class="btn-close" aria-label="Cerrar"></button>
                </div>
                <div class="modal-body">
                    <p>{ "Stats en un futuro" }</p>
                </div>
                <div class="modal-footer">
                    <button onclick={onrestart} type="button" class="btn btn-primary">{ "Reiniciar" }</button>
                </div>
                </div>
            </div>
        </div>
    }
}
