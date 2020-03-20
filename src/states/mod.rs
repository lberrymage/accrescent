pub mod state_prelude {
    pub use amethyst::{GameData, SimpleState, StateData};
}

pub use ingame::Ingame;

mod ingame;
