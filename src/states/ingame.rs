//! The main gameplay state

use super::state_prelude::*;

/// The struct implementing the main gameplay state
pub(crate) struct Ingame;

impl SimpleState for Ingame {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
