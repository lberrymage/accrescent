//! Bindings for the main gameplay state

use std::fmt::{self, Display, Formatter};

use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};

/// Movement bindings
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum IngameAxisBinding {
    /// X-axis (left-right) movement
    X,
    /// Y-axis (vertical) movement
    Y,
    /// Z-axis (forward-backward) movement
    Z,
}

/// Action bindings. Currently there are no actions for this enum.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum IngameActionBinding {
    /// Unused variant. This enum is not yet implemented.
    None,
}

impl Display for IngameAxisBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for IngameActionBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// BindingTypes for the main gameplay state
#[derive(Debug)]
pub(crate) struct IngameBindings;

impl BindingTypes for IngameBindings {
    type Axis = IngameAxisBinding;
    type Action = IngameActionBinding;
}
