use crate::hai_category::HaiCategory;
use std::fmt;
use thiserror::Error;

/// 牌
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

    pub(crate) fn to_dora_str(&self) -> &'static str {
        if self.akadora {
            "$"
        } else {
            ""
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn hai_new() {
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
}
