use crate::{
    agari::Agari, agari_hai::AgariHai, agari_type::AgariType, furo::Furo, hai::Hai,
    jun_tehai::JunTehai, machi_combinations::MachiCombinations, mentsu::Mentsu,
    mentsu_combinations,
};
use std::{cmp::Ordering, fmt, iter, str::FromStr};
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
    pub(crate) fn all_hai(&self) -> impl Iterator<Item = Hai> + '_ {
        self.jun_tehai
            .iter()
            .chain(self.furo.iter().flat_map(|furo| furo.iter()))
            .chain(iter::once(self.agari_hai.hai()))
    }

    fn to_mentsu_combinations(&self) -> Vec<Vec<Mentsu>> {
        let mut tehai = Vec::from(self.jun_tehai.as_slice());
        tehai.push(self.agari_hai.hai());
        tehai.sort();
        mentsu_combinations::combinations(&tehai)
    }

    pub(crate) fn to_agari_combinations(&self) -> Vec<Agari> {
        let mut res = vec![];
        for mentsu in self.to_mentsu_combinations() {
            let machi = MachiCombinations::new(mentsu.iter().copied(), self.agari_hai.hai())
                .map(|machi| Agari::new(self.clone(), mentsu.clone(), machi.1, machi.0));
            res.extend(machi);
        }
        res
    }

    pub(crate) fn is_menzen(&self) -> bool {
        self.furo.iter().all(|furo| furo.is_menzen())
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

    #[wasm_bindgen(getter = junTehai)]
    pub fn jun_tehai_js(&self) -> JunTehai {
        self.jun_tehai.clone()
    }

    #[wasm_bindgen(getter = furo)]
    pub fn furo_js(&self) -> Box<[JsValue]> {
        self.furo.iter().copied().map(JsValue::from).collect()
    }

    #[wasm_bindgen(getter = agariHai)]
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
    #[error("純手牌がありません")]
    NoJunTehai,
    #[error("あがり牌がありません")]
    NoAgariHai,
    #[error("多牌です (牌の数: `{0}`)")]
    Tahai(usize),
    #[error("少牌です (牌の数: `{0}`)")]
    Shohai(usize),
    #[error("純手牌のパースエラー: {0}")]
    JunTehai(#[from] <JunTehai as FromStr>::Err),
    #[error("副露のパースエラー: {0}")]
    Furo(#[from] <Furo as FromStr>::Err),
    #[error("あがり牌のパースエラー: {0}")]
    AgariHai(#[from] <AgariHai as FromStr>::Err),
    #[error("あがり牌が複数あります: `{0}`, `{1}")]
    MultipleAgariHai(AgariHai, AgariHai),
}

impl FromStr for Tehai {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseErrorKind as E;
        let mut chunks = s.split_whitespace();

        let tehai_chunk = chunks.next().ok_or(E::NoJunTehai)?;
        let jun_tehai = JunTehai::from_str(tehai_chunk).map_err(E::from)?;

        let mut furo = vec![];
        let mut agari_hai = None;
        for chunk in chunks {
            if AgariType::is_agari_str(chunk) {
                let new_agari_hai = AgariHai::from_str(chunk).map_err(E::from)?;
                if let Some(old_agari_hai) = agari_hai {
                    return Err(E::MultipleAgariHai(old_agari_hai, new_agari_hai).into());
                }
                agari_hai = Some(new_agari_hai);
            } else {
                furo.push(Furo::from_str(chunk).map_err(E::from)?);
            }
        }

        let agari_hai = agari_hai.ok_or(E::NoAgariHai)?;

        let hai_count = jun_tehai.as_slice().len() + furo.len() * 3 + 1;
        match hai_count.cmp(&14) {
            Ordering::Less => return Err(E::Shohai(hai_count).into()),
            Ordering::Equal => {}
            Ordering::Greater => return Err(E::Tahai(hai_count).into()),
        }

        Ok(Tehai {
            jun_tehai,
            furo,
            agari_hai,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn parse() {
        use ParseErrorKind::*;
        fn ok(s: &str) -> String {
            Tehai::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseErrorKind {
            Tehai::from_str(s).unwrap_err().0
        }
        macro_rules! h {
            ($expected:expr, $($expr:expr),*) => {
                {
                    assert_eq!($expected, [$($expr.to_string()),*].join(""));
                    true
                }
            }
        }

        assert_eq!(
            ok("123m4p <555m <666m <777m ?4p"),
            "123m4p <555m <666m <777m ?4p",
        );
        assert_eq!(
            ok("123m4p <555m !4p <666m <777m"),
            "123m4p <555m <666m <777m !4p",
        );
        assert_eq!(
            ok("12m44p ?3m <555m <666m <777m"),
            "12m44p <555m <666m <777m ?3m",
        );

        assert_matches!(err(""), NoJunTehai);
        assert_matches!(err("123p"), NoAgariHai);
        assert_matches!(err("11122233344455m ?5m"), Tahai(15));
        assert_matches!(err("11122233344m ?4m"), Shohai(12));
        assert_matches!(err("x"), JunTehai(..));
        assert_matches!(err("123m <x"), Furo(..));
        assert_matches!(err("123m ?x"), AgariHai(..));
        assert_matches!(err("123m ?4m !4m"), MultipleAgariHai(a, b) if h!("?4m!4m", a, b));
    }
}
