use crate::{agari::Agari, hai::Hai, hai_vec::HaiVec, hai_with_attr::HaiWithAttr};
use std::{fmt, str::FromStr};
use thiserror::Error;

/// あがり牌
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AgariHai {
    agari: Agari,
    hai: Hai,
}

impl fmt::Display for AgariHai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.agari, self.hai)
    }
}

#[derive(Debug, Error)]
pub(crate) enum ParseError {
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
        use ParseError as E;
        let hai_vec = HaiVec::from_str(s)?;

        if hai_vec.0.len() != 1 {
            return Err(E::InvalidNumberOfHai(hai_vec));
        }

        let hai = hai_vec.0[0];
        match hai {
            HaiWithAttr::Agari(agari, hai) => Ok(Self { agari, hai }),
            _ => Err(E::InvalidHai(hai)),
        }
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
            AgariHai::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseError {
            AgariHai::from_str(s).unwrap_err()
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
