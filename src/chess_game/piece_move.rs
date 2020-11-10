use enum_derive::enum_derive_util;
use enum_derive::EnumFromStr;
use macro_attr::macro_attr;
use macro_attr::macro_attr_impl;

pub struct PieceMove {
    forward: i32,
    left: i32,
    rules: MoveRules,
}

macro_attr! {
#[derive(EnumFromStr!)]
pub enum MoveRules {
    Leap,
    Kill,
    Run,
    PawnFirst,
    Castle,
}
}

impl PieceMove {
    pub fn new(forward: i32, left: i32, rules: MoveRules) -> Self {
        Self {
            forward,
            left,
            rules,
        }
    }
    pub fn new_special(rules: MoveRules) -> Self {
        Self {
            forward: 0,
            left: 0,
            rules,
        }
    }
}
