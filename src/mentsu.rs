use crate::{hai::Hai, hai_vec::HaiVec, hai_with_attr::HaiWithAttr};
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mentsu(MentsuKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MentsuKind {
    Shuntsu([Hai; 3]),
    Kotsu([Hai; 3]),
    Kantsu([Hai; 4]),
    Toitsu([Hai; 2]),
}

impl Mentsu {
    pub(crate) fn shuntsu(hai: [Hai; 3]) -> Self {
        debug_assert!(hai[1].is_next_to(&hai[0]) && hai[2].is_next_to(&hai[1]));
        Self(MentsuKind::Shuntsu(hai))
    }

    pub(crate) fn kotsu(hai: [Hai; 3]) -> Self {
        debug_assert!(hai[0].is_same(&hai[1]) && hai[0].is_same(&hai[2]));
        Self(MentsuKind::Kotsu(hai))
    }

    pub(crate) fn kantsu(hai: [Hai; 4]) -> Self {
        debug_assert!(
            hai[0].is_same(&hai[1]) && hai[0].is_same(&hai[2]) && hai[0].is_same(&hai[3])
        );
        Self(MentsuKind::Kantsu(hai))
    }

    pub(crate) fn toitsu(hai: [Hai; 2]) -> Self {
        debug_assert!(hai[0].is_same(&hai[1]));
        Self(MentsuKind::Toitsu(hai))
    }

    fn to_hai_vec(&self) -> HaiVec {
        let t = HaiWithAttr::FromTehai;
        match self.0 {
            MentsuKind::Shuntsu([h0, h1, h2]) | MentsuKind::Kotsu([h0, h1, h2]) => {
                HaiVec::new([t(h0), t(h1), t(h2)])
            }
            MentsuKind::Kantsu([h0, h1, h2, h3]) => HaiVec::new([t(h0), t(h1), t(h2), t(h3)]),
            MentsuKind::Toitsu([h0, h1]) => HaiVec::new([t(h0), t(h1)]),
        }
    }
}

#[wasm_bindgen]
impl Mentsu {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_js(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Mentsu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hai_vec())
    }
}
