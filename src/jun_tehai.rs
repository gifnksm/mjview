use crate::{hai::Hai, hai_image::HaiImage, hai_vec::HaiVec, hai_with_attr::HaiWithAttr};
use std::{fmt, str::FromStr};
use thiserror::Error;
use wasm_bindgen::prelude::*;

/// 純手牌 (狭義の手牌。手牌のうち副露でないもの)
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct JunTehai(Vec<Hai>);

impl fmt::Display for JunTehai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hai_vec())
    }
}

impl JunTehai {
    pub(crate) fn as_slice(&self) -> &[Hai] {
        &self.0
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = Hai> + '_ {
        self.0.iter().copied()
    }

    fn to_hai_vec(&self) -> HaiVec {
        HaiVec::new(
            self.0
                .iter()
                .copied()
                .map(HaiWithAttr::FromTehai)
                .collect::<Vec<_>>(),
        )
    }

    fn to_image(&self) -> Vec<HaiImage> {
        self.0.iter().copied().map(HaiImage::normal).collect()
    }
}

#[wasm_bindgen]
impl JunTehai {
    #[wasm_bindgen(js_name = "toHaiArray")]
    pub fn to_hai_array_js(&self) -> Box<[JsValue]> {
        self.as_slice().iter().copied().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_js(&self) -> String {
        self.to_string()
    }

    #[wasm_bindgen(js_name = "toImage")]
    pub fn to_image_js(&self) -> Box<[JsValue]> {
        self.to_image().into_iter().map(JsValue::from).collect()
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] ParseErrorKind);

#[derive(Debug, Error)]
enum ParseErrorKind {
    #[error("不正な牌が含まれています: `{0}`")]
    InvalidHai(HaiWithAttr),
    #[error(transparent)]
    HaiVec(#[from] <HaiVec as FromStr>::Err),
}

impl FromStr for JunTehai {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseErrorKind as E;
        let hai_vec = HaiVec::from_str(s)
            .map_err(E::from)?
            .0
            .into_iter()
            .map(|hai| match hai {
                HaiWithAttr::FromTehai(hai) => Ok(hai),
                _ => Err(E::InvalidHai(hai)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::from)?;
        Ok(Self(hai_vec))
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
            JunTehai::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseErrorKind {
            JunTehai::from_str(s).unwrap_err().0
        }
        macro_rules! h {
            ($expected:expr, $($expr:expr),*) => {
                {
                    assert_eq!($expected, [$($expr.to_string()),*].join(""));
                    true
                }
            }
        }

        assert_eq!(ok("1p35p2s4m6s3m79m"), "135p2s4m6s379m");

        assert_matches!(err("<1^2j>3+4j"), InvalidHai(hai) if h!("<1j", hai));
        assert_matches!(err("12+3p"), InvalidHai(hai) if h!("+3p", hai));
        assert_matches!(err("4!56p"), InvalidHai(hai) if h!("!5p", hai));
        assert_matches!(err("0m"), HaiVec(_));
    }
}
