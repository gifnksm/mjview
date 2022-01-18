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
    #[wasm_bindgen(getter, js_name = "type")]
    pub fn type_js(&self) -> String {
        match self.0 {
            HaiImageKind::Normal(_) => "normal",
            HaiImageKind::Sideways(_) => "sideways",
            HaiImageKind::Hidden(_) => "hidden",
            HaiImageKind::Stack { .. } => "stack",
        }
        .into()
    }

    #[wasm_bindgen(getter, js_name = "hai")]
    pub fn hai_js(&self) -> Option<Hai> {
        match self.0 {
            HaiImageKind::Normal(hai) => Some(hai),
            HaiImageKind::Sideways(hai) => Some(hai),
            HaiImageKind::Hidden(hai) => Some(hai),
            HaiImageKind::Stack { .. } => None,
        }
    }

    #[wasm_bindgen(getter, js_name = "top")]
    pub fn top_js(&self) -> Option<Hai> {
        if let HaiImageKind::Stack { top, .. } = self.0 {
            Some(top)
        } else {
            None
        }
    }

    #[wasm_bindgen(getter, js_name = "bottom")]
    pub fn bottom_js(&self) -> Option<Hai> {
        if let HaiImageKind::Stack { bottom, .. } = self.0 {
            Some(bottom)
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_js(self) -> String {
        match self.0 {
            HaiImageKind::Normal(hai) => hai.to_string(),
            HaiImageKind::Sideways(hai) => format!("y_{}", hai),
            HaiImageKind::Hidden(_) => "_".into(),
            HaiImageKind::Stack { bottom, top } => {
                format!("{{top:y_{}}}{{bottom:y_{}}}", top, bottom)
            }
        }
    }
}
