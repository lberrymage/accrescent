//! The collection of possible game states

mod ingame;

/// Contains common types used when implememnting a [`State`]
///
/// [`State`]: https://docs.amethyst.rs/stable/amethyst/trait.State.html
mod state_prelude {
    pub(super) use amethyst::{GameData, SimpleState, StateData};
}

pub(crate) use ingame::Ingame;
