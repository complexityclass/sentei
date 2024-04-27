pub mod app;

use lazy_static::lazy_static;
use wasm_bindgen::prelude::wasm_bindgen;

pub use crux_core::{bridge::Bridge, Core, Request};

pub use app::*;

extern crate go_bot;

// TODO hide this plumbing

uniffi::include_scaffolding!("shared");

lazy_static! {
    static ref CORE: Bridge<Effect, Counter> = Bridge::new(Core::new::<Capabilities>());
}

#[wasm_bindgen]
pub fn process_event(data: &[u8]) -> Vec<u8> {
    CORE.process_event(data)
}

#[wasm_bindgen]
pub fn handle_response(uuid: &[u8], data: &[u8]) -> Vec<u8> {
    CORE.handle_response(uuid, data)
}

#[wasm_bindgen]
pub fn view() -> Vec<u8> {
    CORE.view()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_model() {
        let vm = ViewModel {
            count: "6".to_string(),
        };
        assert_eq!(vm.count, "6");
    }

    #[test]
    fn test_bot() {
        let game = go_bot::Game::new(true);
        assert_eq!(game.human.other(), game.ai);
    }
}
