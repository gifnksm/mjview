use crate::hai::Hai;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HaiImage(HaiImageKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HaiImageKind {
    Normal(Hai),
    Sideways(Hai),
    Hidden(Hai),
    Stack { bottom: Hai, top: Hai },
}

impl HaiImage {
    pub(crate) fn normal(hai: Hai) -> Self {
        Self(HaiImageKind::Normal(hai))
    }

    pub(crate) fn sideways(hai: Hai) -> Self {
        Self(HaiImageKind::Sideways(hai))
    }

    pub(crate) fn hidden(hai: Hai) -> Self {
        Self(HaiImageKind::Hidden(hai))
    }

    pub(crate) fn stack(bottom: Hai, top: Hai) -> Self {
        Self(HaiImageKind::Stack { bottom, top })
    }
}

#[wasm_bindgen]
impl HaiImage {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_js(&self) -> String {
        match self.0 {
            HaiImageKind::Normal(hai) => hai.to_string(),
            HaiImageKind::Sideways(hai) => format!("y_{}", hai.to_string()),
            HaiImageKind::Hidden(_) => "_".into(),
            HaiImageKind::Stack { bottom, top } => format!("y_{}/y_{}", top, bottom),
        }
    }
}
