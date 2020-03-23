use std::fmt::{Display, Formatter, Result};

use amethyst::{
    input::BindingTypes
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub enum AxisBinding {
    Vertical(u32),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub enum ActionBinding {}

#[derive(Debug)]
pub struct PongBindings;

impl Display for AxisBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ActionBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl BindingTypes for PongBindings {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
