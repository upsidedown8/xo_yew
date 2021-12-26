use xo::board::{Board, GameState};
use yew::prelude::*;

pub enum Msg {
    SquareClick(usize),
    Reset,
}

pub struct App {
    board: Board,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            board: Board::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SquareClick(square) => {
                let update = self.board.make_move(square).is_ok();

                if update {
                    if let Ok(best_move) = xo::ai::best_move(&self.board) {
                        self.board.make_move(best_move).ok();
                    }
                }

                update
            }
            Msg::Reset => {
                self.board = Board::default();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = self.board.state();
        let next_player = self.board.next_player();
        let mut squares = [None; 9];

        squares.iter_mut().enumerate().for_each(|(pos, elem)| {
            *elem = self.board.get_square(pos);
        });

        let callback = ctx.link().callback(Msg::SquareClick);

        let is_empty = (0..9).all(|pos| self.board.get_square(pos).is_none());

        html! {
            <>
                <p class="game-status">

                { match state {
                    GameState::Indeterminate => {
                        format!("{} to play", next_player)
                    },
                    GameState::Winner(player) => {
                        format!("{} has won!", player)
                    },
                    GameState::Draw => {
                        String::from("draw!")
                    },
                } }

                </p>

                <crate::board::Board {squares} {callback} />

                if !is_empty {
                    <button
                        class="button btn-reset is-large is-outlined"
                        onclick={ctx.link().callback(|_| Msg::Reset)}
                    >
                        { "reset" }
                    </button>
                }
            </>
        }
    }
}
