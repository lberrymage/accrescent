use super::state_prelude::*;

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
