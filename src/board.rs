use crate::square::Square;
use xo::board::Player;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub squares: [Option<Player>; 9],
    pub callback: Callback<usize>,
}

pub struct Board;

impl Component for Board {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.props().callback.clone();
        let mut i = 0;

        html! {
            <div class="board">
            {for ctx.props().squares
                .chunks(3)
                .map(|row| html! {
                    <div class="board-row">
                        {for row
                            .iter()
                            .copied()
                            .map(|value| {

                                let onclick = {
                                    let callback = callback.clone();
                                    Callback::from(move |_| {
                                        callback.emit(i)
                                    })
                                };

                                i += 1;

                                html! {
                                    <Square {value} {onclick} />
                                }
                            })
                        }
                    </div>
                })}
            </div>
        }
    }
}
