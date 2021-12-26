use xo::board::Player;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    pub value: Option<Player>,
}

pub struct Square;

impl Component for Square {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = &ctx.props().onclick;

        html! {
            <button class={"square"} {onclick}>
                {match ctx.props().value {
                    Some(Player::X) => 'X',
                    Some(Player::O) => 'O',
                    None => ' ',
                }}
            </button>
        }
    }
}
