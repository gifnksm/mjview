use crate::{
    hai_builder::{Error as ParseError, HaiBuilder},
    hai_category::HaiCategory,
    hai_with_attr::HaiWithAttr,
};
use std::{fmt, str::FromStr};
use thiserror::Error;
use wasm_bindgen::prelude::*;

/// 牌
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hai {
    category: HaiCategory,
    number: u8,
    akadora: bool,
}

impl fmt::Display for Hai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.number, self.to_dora_str(), self.category)
    }
}

impl FromStr for Hai {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut builder = HaiBuilder::new(false);
        builder.eat_whole_str(s)?;
        let hai = match builder.build()? {
            HaiWithAttr::FromTehai(hai) => hai,
            _ => unreachable!(),
        };
        Ok(hai)
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub(crate) struct NewError(#[from] NewErrorKind);

#[derive(Debug, Error)]
enum NewErrorKind {
    #[error("invalid hai: `{number}{category}`")]
    InvalidNumber { number: u8, category: HaiCategory },
    #[error("invalid akadora: `{number}${category}`")]
    InvalidAkadora { number: u8, category: HaiCategory },
}

impl Hai {
    pub(crate) fn try_new(
        category: HaiCategory,
        number: u8,
        akadora: bool,
    ) -> Result<Self, NewError> {
        use {HaiCategory::*, NewErrorKind::*};
        let range = match category {
            Manzu => (1..=9), // 1-9
            Pinzu => (1..=9),
            Souzu => (1..=9),
            Jihai => (1..=7), // 1-7: 東南西北白發中
        };
        if !range.contains(&number) {
            return Err(InvalidNumber { number, category }.into());
        }
        if akadora && (number != 5 || category == Jihai) {
            return Err(InvalidAkadora { number, category }.into());
        }
        Ok(Self {
            category,
            number,
            akadora,
        })
    }

    pub(crate) fn category(&self) -> HaiCategory {
        self.category
    }
    pub(crate) fn number(&self) -> u8 {
        self.number
    }

    pub(crate) fn is_same(&self, other: &Hai) -> bool {
        self.category == other.category && self.number == other.number
    }

    pub(crate) fn is_next_to(&self, other: &Hai) -> bool {
        self.category == other.category && self.number == other.number + 1
    }

    /// 么九牌か否か
    pub(crate) fn is_yaochuhai(&self) -> bool {
        self.category == HaiCategory::Jihai || self.number == 1 || self.number == 9
    }

    /// 三元牌か否か
    pub(crate) fn is_sangenpai(&self) -> bool {
        self.category == HaiCategory::Jihai && (5..=7).contains(&self.number)
    }

    pub(crate) fn to_dora_str(&self) -> &'static str {
        if self.akadora {
            "$"
        } else {
            ""
        }
    }
}

#[wasm_bindgen]
impl Hai {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_js(&self) -> String {
        self.to_string()
    }

    #[wasm_bindgen(js_name = "fromStr")]
    pub fn from_str_js(s: &str) -> Result<Hai, JsValue> {
        let res = Self::from_str(s).map_err(|e| e.to_string())?;
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hai_builder::ErrorKind as ParseErrorKind;
    use assert_matches::assert_matches;

    #[test]
    fn new() {
        use HaiCategory::*;
        fn ok(category: HaiCategory, number: u8, akadora: bool) {
            assert_eq!(
                Hai::try_new(category, number, akadora).unwrap(),
                Hai {
                    category,
                    number,
                    akadora
                }
            );
        }
        fn invalid_number(category: HaiCategory, number: u8, akadora: bool) {
            assert_matches!(
                Hai::try_new(category, number, akadora).unwrap_err().0,
                NewErrorKind::InvalidNumber { number: n, category: c } if n == number && c == category
            );
        }
        ok(Manzu, 8, false);
        ok(Souzu, 4, false);
        ok(Pinzu, 5, true);
        ok(Jihai, 5, false);
        invalid_number(Manzu, 10, false);
        invalid_number(Souzu, 10, false);
        invalid_number(Pinzu, 10, false);
        invalid_number(Jihai, 0, false);
        invalid_number(Jihai, 8, false);
    }

    #[test]
    fn parse() {
        use ParseErrorKind::*;
        fn ok(s: &str) -> String {
            Hai::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseErrorKind {
            Hai::from_str(s).unwrap_err().into()
        }
        macro_rules! h {
            ($expected:expr, $($expr:expr),*) => {
                {
                    assert_eq!($expected, [$($expr.to_string()),*].join(""));
                    true
                }
            }
        }
        assert_eq!(ok("1p"), "1p");
        assert_eq!(ok("5$m"), "5$m");

        assert_matches!(err(""), NumberNotFound);
        assert_matches!(err("p"), InvalidChar('p'));
        assert_matches!(err("1"), CategoryNotFound);
        assert_matches!(err("1pm"), MultipleCategory(a, b) if h!("pm", a, b));
        assert_matches!(err("1p2p"), InvalidChar('2'));
        assert_matches!(err("12p"), InvalidChar('2'));
        assert_matches!(err("<1p"), InvalidChar('<'));
        assert_matches!(err("1$$p"), MultipleDora);
    }
}
