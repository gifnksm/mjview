use crate::{hai::Hai, hai_vec::HaiVec, hai_with_attr::HaiWithAttr};
use std::{fmt, str::FromStr};
use thiserror::Error;

/// 純手牌 (狭義の手牌。手牌のうち副露でないもの)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct JunTehai(Vec<Hai>);

impl fmt::Display for JunTehai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hai_vec())
    }
}

impl JunTehai {
    pub(crate) fn as_vec(&self) -> &[Hai] {
        &self.0
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
}

#[derive(Debug, Error)]
pub(crate) enum ParseError {
    #[error("invalid hai found: `{0}`")]
    InvalidHai(HaiWithAttr),
    #[error(transparent)]
    HaiVec(#[from] <HaiVec as FromStr>::Err),
}

impl FromStr for JunTehai {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseError as E;
        let hai_vec = HaiVec::from_str(s)?
            .0
            .into_iter()
            .map(|hai| match hai {
                HaiWithAttr::FromTehai(hai) => Ok(hai),
                _ => Err(E::InvalidHai(hai)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(hai_vec))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn parse() {
        use ParseError::*;
        fn ok(s: &str) -> String {
            JunTehai::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseError {
            JunTehai::from_str(s).unwrap_err()
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
