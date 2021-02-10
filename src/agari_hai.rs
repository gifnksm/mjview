use crate::{
    agari::Agari, hai::Hai, hai_image::HaiImage, hai_vec::HaiVec, hai_with_attr::HaiWithAttr,
};
use std::{fmt, str::FromStr};
use thiserror::Error;
use wasm_bindgen::prelude::*;

/// あがり牌
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AgariHai {
    agari: Agari,
    hai: Hai,
}

impl fmt::Display for AgariHai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.agari, self.hai)
    }
}

impl AgariHai {
    pub(crate) fn hai(&self) -> Hai {
        self.hai
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] ParseErrorKind);

#[derive(Debug, Error)]
enum ParseErrorKind {
    #[error("invalid hai found: `{0}`")]
    InvalidHai(HaiWithAttr),
    #[error("invalid number of hai: `{0}`")]
    InvalidNumberOfHai(HaiVec),
    #[error(transparent)]
    HaiVec(#[from] <HaiVec as FromStr>::Err),
}

impl FromStr for AgariHai {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseErrorKind as E;
        let hai_vec = HaiVec::from_str(s).map_err(E::from)?;

        if hai_vec.0.len() != 1 {
            return Err(E::InvalidNumberOfHai(hai_vec).into());
        }

        let hai = hai_vec.0[0];
        match hai {
            HaiWithAttr::Agari(agari, hai) => Ok(Self { agari, hai }),
            _ => Err(E::InvalidHai(hai).into()),
        }
    }
}

impl AgariHai {
    fn to_image(&self) -> HaiImage {
        HaiImage::normal(self.hai)
    }
}

#[wasm_bindgen]
impl AgariHai {
    #[wasm_bindgen(getter, js_name = "agari")]
    pub fn agari_js(&self) -> String {
        self.agari.to_str().into()
    }

    #[wasm_bindgen(js_name = "toImage")]
    pub fn to_image_js(&self) -> HaiImage {
        self.to_image()
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
            AgariHai::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseErrorKind {
            AgariHai::from_str(s).unwrap_err().0
        }
        macro_rules! h {
            ($expected:expr, $($expr:expr),*) => {
                {
                    assert_eq!($expected, [$($expr.to_string()),*].join(""));
                    true
                }
            }
        }

        assert_eq!(ok("!1p"), "!1p");
        assert_eq!(ok("?1p"), "?1p");

        assert_matches!(err("!1?2!3?4j"), InvalidNumberOfHai(hai) if h!("!1?2!3?4j", hai));
        assert_matches!(err("8m"), InvalidHai(hai) if h!("8m", hai));
        assert_matches!(err(">3j"), InvalidHai(hai) if h!(">3j", hai));
        assert_matches!(err("+3p"), InvalidHai(hai) if h!("+3p", hai));
        assert_matches!(err("0m"), HaiVec(_));
    }
}
