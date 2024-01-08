use stylist::yew::styled_component;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FinishScreenProps {
    pub is_win: bool,
    pub word: String,
    pub attempt: usize,
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

    let tries = if props.is_win {
        Some(props.attempt)
    } else {
        None
    };
    add_win(tries);
    let wins = get_wins();
    let (winned, graph_data) = wins.calculate_values();
    let ratio = (winned as f64 / wins.total as f64) * 100.0;

    html! {
        <>
        <div class="modal" tabindex="-1" style="display: block;">
            <div class="modal-dialog">
                <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title">
                        if props.is_win {
                            { format!("Has ganado! ({})", props.word) }
                        } else {
                            { format!("Has perdido ({})", props.word) }
                        }
                    </h5>
                    <button onclick={onclose} type="button" class="btn-close" aria-label="Cerrar"></button>
                </div>
                <div class="modal-body">
                    <p>{format!("Has ganado: {} partidas ({:.2}%)", winned, ratio)}</p>
                    <canvas id="stats">{ "No se han podido cargar las estadísticas" }</canvas>
                </div>
                <div class="modal-footer">
                    <button onclick={onrestart} type="button" class="btn btn-primary">{ "Reiniciar" }</button>
                </div>
                </div>
            </div>
        </div>

        <script>
        {format!("{}{}{}", r#"
        setTimeout(function() {

        new Chart("stats",  {
            type: 'bar',
            data: {
              labels: ['1', '2', '3', '4', '5', '6', '>6'],
              datasets: [{
                data: ["#, graph_data ,r#"],
                backgroundColor: '#0d6efd',
              }],
            },
            options: {
              indexAxis: 'y',
              responsive: true,
              plugins: {
                legend: {
                  display: false
                }
              },
            }
        });

        }, 100);
        "#)}
        </script>
        </>
    }
}

struct Wins {
    one: u32,
    two: u32,
    three: u32,
    four: u32,
    five: u32,
    six: u32,
    total: u32
}

impl Wins {
    fn calculate_values(&self) -> (u32, String) {
        let winned = self.one + self.two + self.three + self.four + self.five + self.six;
        let string = format!("{},{},{},{},{},{},{}", self.one, self.two, self.three, self.four, self.five, self.six, (self.total - winned));

        (winned, string)
    }
}

fn get_wins() -> Wins {
    let one = LocalStorage::get("one").unwrap_or_default();
    let two = LocalStorage::get("two").unwrap_or_default();
    let three = LocalStorage::get("three").unwrap_or_default();
    let four = LocalStorage::get("four").unwrap_or_default();
    let five = LocalStorage::get("five").unwrap_or_default();
    let six = LocalStorage::get("six").unwrap_or_default();
    let total = LocalStorage::get("total").unwrap_or_default();

    Wins {
        one,
        two,
        three,
        four,
        five,
        six,
        total,
    }
}

fn add_win(tries: Option<usize>) {
    let wins = get_wins();

    if let Some(att) = tries {
        match att {
            1 => LocalStorage::set("one", wins.one + 1).unwrap(),
            2 => LocalStorage::set("two", wins.two + 1).unwrap(),
            3 => LocalStorage::set("three", wins.three + 1).unwrap(),
            4 => LocalStorage::set("four", wins.four + 1).unwrap(),
            5 => LocalStorage::set("five", wins.five + 1).unwrap(),
            6 => LocalStorage::set("six", wins.six + 1).unwrap(),
            n => gloo::console::error!(format!("Has ganado en un numero distinto de intentos ({})", n))
        }
    }
    LocalStorage::set("total", wins.total + 1).ok();
}