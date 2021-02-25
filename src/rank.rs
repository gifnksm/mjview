use std::fmt;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rank(RankKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RankKind {
    Fan(u32),
    Yakuman(u32),
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            RankKind::Fan(n) => write!(f, "{}飜", n),
            RankKind::Yakuman(1) => write!(f, "役満"),
            RankKind::Yakuman(n) => write!(f, "{}倍役満", n),
        }
    }
}

impl Rank {
    pub(crate) fn new_fan(fan: u32) -> Self {
        Self(RankKind::Fan(fan))
    }

    pub(crate) fn new_yakuman(count: u32) -> Self {
        Self(RankKind::Yakuman(count))
    }

    pub(crate) fn kind(&self) -> &RankKind {
        &self.0
    }
}

#[wasm_bindgen]
impl Rank {
    #[wasm_bindgen(getter = fan)]
    pub fn fan_js(&self) -> Option<u32> {
        match self.0 {
            RankKind::Fan(n) => Some(n),
            _ => None,
        }
    }

    #[wasm_bindgen(getter = yakuman)]
    pub fn yakuman_js(&self) -> Option<u32> {
        match self.0 {
            RankKind::Yakuman(n) => Some(n),
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string_js(&self) -> String {
        self.to_string()
    }
}
