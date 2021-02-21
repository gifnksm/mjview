use crate::{
    agari::Agari, agari_hai::AgariHai, agari_type::AgariType, furo::Furo, jun_tehai::JunTehai,
    machi_combinations::MachiCombinations, mentsu::Mentsu, mentsu_combinations,
};
use std::{fmt, str::FromStr};
use thiserror::Error;
use wasm_bindgen::prelude::*;

/// 手牌
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tehai {
    jun_tehai: JunTehai,
    furo: Vec<Furo>,
    agari_hai: AgariHai,
}

impl fmt::Display for Tehai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.jun_tehai)?;
        for furo in &self.furo {
            write!(f, " {}", furo)?;
        }
        write!(f, " {}", self.agari_hai)?;
        Ok(())
    }
}

impl Tehai {
    fn to_mentsu_combinations(&self) -> Vec<Vec<(Mentsu, u16)>> {
        let mut tehai = Vec::from(self.jun_tehai.as_slice());
        tehai.push(self.agari_hai.hai());
        tehai.sort();
        mentsu_combinations::combinations(&tehai)
    }

    pub(crate) fn to_agari_combinations(&self) -> Vec<Agari> {
        let mut res = vec![];
        for mentsu in self.to_mentsu_combinations() {
            let machi = MachiCombinations::new(
                mentsu[..].iter().map(|&(mentsu, _)| mentsu),
                self.agari_hai.hai(),
            )
            .map(|machi| Agari::new(self.clone(), mentsu.clone(), machi));
            res.extend(machi);
        }
        res
    }

    pub(crate) fn furo(&self) -> &[Furo] {
        &self.furo
    }

    pub(crate) fn agari_hai(&self) -> AgariHai {
        self.agari_hai
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

    #[wasm_bindgen(getter, js_name = "agariHai")]
    pub fn agari_hai_js(&self) -> AgariHai {
        self.agari_hai
    }

    #[wasm_bindgen(js_name = "toAgariCombinations")]
    pub fn to_agari_combinations_js(&self) -> Box<[JsValue]> {
        self.to_agari_combinations()
            .into_iter()
            .map(JsValue::from)
            .collect()
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
            if AgariType::is_agari_str(chunk) {
                break;
            }
            let chunk = chunks.next().unwrap();
            furo.push(Furo::from_str(chunk).map_err(E::from)?);
        }

        let agari_chunk = chunks.next().ok_or(E::NoAgariHai)?;
        let agari_hai = AgariHai::from_str(agari_chunk).map_err(E::from)?;

        let hai_count = jun_tehai.as_slice().len() + furo.len() * 3 + 1;
        if hai_count != 14 {
            return Err(E::InvalidHaiCount(hai_count).into());
        }

        Ok(Tehai {
            jun_tehai,
            furo,
            agari_hai,
        })
    }
}
