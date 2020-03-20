use super::state_prelude::*;

pub struct Ingame;

impl SimpleState for Ingame {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
