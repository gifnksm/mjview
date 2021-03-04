use crate::{hai::Hai, jun_tehai::JunTehai};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Env {
    pub(crate) tenho: bool,
    pub(crate) richi: bool,
    pub(crate) daburi: bool,
    pub(crate) ippatsu: bool,
    pub(crate) rinshan: bool,
    pub(crate) haitei: bool,
    pub(crate) bakaze: Hai,
    pub(crate) jikaze: Hai,
    pub(crate) dora: Vec<Hai>,
    pub(crate) uradora: Vec<Hai>,
}

impl Env {
    #[cfg(test)]
    pub(crate) fn new_empty(bakaze: Hai, jikaze: Hai) -> Env {
        Env {
            tenho: false,
            richi: false,
            daburi: false,
            ippatsu: false,
            rinshan: false,
            haitei: false,
            bakaze,
            jikaze,
            dora: vec![],
            uradora: vec![],
        }
    }
}

#[wasm_bindgen]
impl Env {
    #[wasm_bindgen(constructor)]
    pub fn new_js() -> Env {
        Env {
            tenho: false,
            richi: false,
            daburi: false,
            ippatsu: false,
            rinshan: false,
            haitei: false,
            bakaze: Hai::from_str("1j").unwrap(),
            jikaze: Hai::from_str("1j").unwrap(),
            dora: vec![],
            uradora: vec![],
        }
    }

    #[wasm_bindgen(getter)]
    pub fn tenho(&self) -> bool {
        self.tenho
    }

    #[wasm_bindgen(setter)]
    pub fn set_tenho(&mut self, value: bool) {
        self.tenho = value;
    }

    #[wasm_bindgen(getter)]
    pub fn richi(&self) -> bool {
        self.richi
    }

    #[wasm_bindgen(setter)]
    pub fn set_richi(&mut self, value: bool) {
        self.richi = value;
    }

    #[wasm_bindgen(getter = daburi)]
    pub fn daburi(&self) -> bool {
        self.daburi
    }

    #[wasm_bindgen(setter = daburi)]
    pub fn set_daburi(&mut self, value: bool) {
        self.daburi = value;
    }

    #[wasm_bindgen(getter)]
    pub fn ippatsu(&self) -> bool {
        self.ippatsu
    }

    #[wasm_bindgen(setter)]
    pub fn set_ippatsu(&mut self, value: bool) {
        self.ippatsu = value;
    }

    #[wasm_bindgen(getter)]
    pub fn rinshan(&self) -> bool {
        self.rinshan
    }

    #[wasm_bindgen(setter)]
    pub fn set_rinshan(&mut self, value: bool) {
        self.rinshan = value;
    }

    #[wasm_bindgen(getter)]
    pub fn haitei(&self) -> bool {
        self.haitei
    }

    #[wasm_bindgen(setter)]
    pub fn set_haitei(&mut self, value: bool) {
        self.haitei = value;
    }

    #[wasm_bindgen(getter)]
    pub fn bakaze(&self) -> Hai {
        self.bakaze
    }

    #[wasm_bindgen(setter = bakaze)]
    pub fn set_bakaze_js(&mut self, value: &Hai) {
        self.bakaze = *value;
    }

    #[wasm_bindgen(getter)]
    pub fn jikaze(&self) -> Hai {
        self.jikaze
    }

    #[wasm_bindgen(setter = jikaze)]
    pub fn set_jikaze_js(&mut self, value: &Hai) {
        self.jikaze = *value;
    }

    #[wasm_bindgen(js_name = "setDora")]
    pub fn set_dora_js(&mut self, value: String) -> Result<(), JsValue> {
        let tehai = JunTehai::from_str(&value).map_err(|e| e.to_string())?;
        self.dora = tehai.as_slice().into();
        Ok(())
    }

    #[wasm_bindgen(getter = doraCount)]
    pub fn dora_count_js(&self) -> usize {
        self.dora.len()
    }

    #[wasm_bindgen(getter = dora)]
    pub fn dora_js(&self) -> Box<[JsValue]> {
        self.dora.iter().copied().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "setUradora")]
    pub fn set_uradora_js(&mut self, value: String) -> Result<(), JsValue> {
        let tehai = JunTehai::from_str(&value).map_err(|e| e.to_string())?;
        self.uradora = tehai.as_slice().into();
        Ok(())
    }

    #[wasm_bindgen(getter = uradoraCount)]
    pub fn uradora_count_ja(&self) -> usize {
        self.uradora.len()
    }

    #[wasm_bindgen(getter = uradora)]
    pub fn uradora_js(&self) -> Box<[JsValue]> {
        self.uradora.iter().copied().map(JsValue::from).collect()
    }
}
