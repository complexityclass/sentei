use crux_core::{render::Render, App};
use serde::{Deserialize, Serialize};
use go_bot::{DemoGame, Game, Play, Position};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    MakeMove
}

#[derive(Default)]
pub struct Model {
    game: DemoGame,
    play: Option<Play>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ViewModel {
    pub isPlay: bool,
    pub isWhite: bool,
    pub row: i32,
    pub col: i32,
    pub isEnd: bool
}

impl ViewModel {
    pub fn not_play() -> Self {
        ViewModel {
            isPlay: false,
            isWhite: false,
            row: 0,
            col: 0,
            isEnd: false
        }
    }
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
#[effect(app = "Counter")]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct Counter;

impl App for Counter {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::MakeMove => {
                model.play = Some(model.game.next_play())
            }
        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        if let Some(play) = &model.play {
            let default = Position::default();
            let position = play.position.as_ref().unwrap_or(&default);

            ViewModel {
                isPlay: true,
                isWhite: play.is_white,
                row: position.row,
                col: position.col,
                isEnd: play.is_end,
            }
        } else {
            ViewModel::not_play()
        }
    }
}