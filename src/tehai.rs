use crate::{
    agari::Agari, agari_hai::AgariHai, furo::Furo, jun_tehai::JunTehai, mentsu::Mentsu,
    mentsu_combinations,
};
use js_sys::Array;
use std::{fmt, str::FromStr};
use thiserror::Error;
use wasm_bindgen::prelude::*;

/// 手牌
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tehai {
    jun_tehai: JunTehai,
    furo: Vec<Furo>,
    agari: AgariHai,
}

impl fmt::Display for Tehai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.jun_tehai)?;
        for furo in &self.furo {
            write!(f, " {}", furo)?;
        }
        write!(f, " {}", self.agari)?;
        Ok(())
    }
}

impl Tehai {
    fn to_mentsu_combinations(&self) -> Vec<Vec<(Mentsu, u16)>> {
        let mut tehai = Vec::from(self.jun_tehai.as_slice());
        tehai.push(self.agari.hai());
        tehai.sort();
        mentsu_combinations::combinations(&tehai)
    }
}

#[wasm_bindgen]
impl Tehai {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_js(&self) -> String {
        self.to_string()
    }

    #[wasm_bindgen(getter, js_name = "junTehai")]
    pub fn jun_tehai_js(&self) -> JunTehai {
        self.jun_tehai.clone()
    }

    #[wasm_bindgen(getter, js_name = "furo")]
    pub fn furo_js(&self) -> Box<[JsValue]> {
        self.furo.iter().copied().map(JsValue::from).collect()
    }

    #[wasm_bindgen(getter, js_name = "agari")]
    pub fn agari_js(&self) -> AgariHai {
        self.agari
    }

    #[wasm_bindgen(js_name = "toMentsuCombinations")]
    pub fn to_mentsu_combinations_js(&self) -> Array {
        self.to_mentsu_combinations()
            .into_iter()
            .map(|comb| {
                comb.into_iter()
                    .map(|(mentsu, _bits)| mentsu)
                    .map(JsValue::from)
                    .collect::<Array>()
            })
            .map(JsValue::from)
            .collect::<Array>()
    }

    #[wasm_bindgen(js_name = "fromStr")]
    pub fn from_str_js(s: &str) -> Result<Tehai, JsValue> {
        let res = Self::from_str(s).map_err(|e| e.to_string())?;
        Ok(res)
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] ParseErrorKind);

#[derive(Debug, Error)]
enum ParseErrorKind {
    #[error("no jun-tehai found")]
    NoJunTehai,
    #[error("no agari-hai found")]
    NoAgariHai,
    #[error("invalid hai count: `{0}`")]
    InvalidHaiCount(usize),
    #[error("jun-tehai: {0}")]
    JunTehai(#[from] <JunTehai as FromStr>::Err),
    #[error("furo: {0}")]
    Furo(#[from] <Furo as FromStr>::Err),
    #[error("agari-hai: {0}")]
    AgariHai(#[from] <AgariHai as FromStr>::Err),
}

impl FromStr for Tehai {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseErrorKind as E;
        let mut chunks = s.split_whitespace().peekable();

        let tehai_chunk = chunks.next().ok_or(E::NoJunTehai)?;
        let jun_tehai = JunTehai::from_str(tehai_chunk).map_err(E::from)?;

        let mut furo = vec![];
        while let Some(chunk) = chunks.peek() {
            if Agari::is_agari_str(chunk) {
                break;
            }
            let chunk = chunks.next().unwrap();
            furo.push(Furo::from_str(chunk).map_err(E::from)?);
        }

        let agari_chunk = chunks.next().ok_or(E::NoAgariHai)?;
        let agari = AgariHai::from_str(agari_chunk).map_err(E::from)?;

        let hai_count = jun_tehai.as_slice().len() + furo.len() * 3 + 1;
        if hai_count != 14 {
            return Err(E::InvalidHaiCount(hai_count).into());
        }

        Ok(Tehai {
            jun_tehai,
            furo,
            agari,
        })
    }
}
