use crate::{
    hai::{self, Hai},
    hai_category::HaiCategory,
    hai_with_attr::HaiWithAttr,
    tacha::Tacha,
};
use enum_iterator::IntoEnumIterator;
use std::{fmt, iter::FromIterator, str::FromStr};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct HaiVec(pub(crate) Vec<HaiWithAttr>);

impl fmt::Display for HaiVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut last_category = None;
        for hai in &self.0 {
            if let Some(l) = last_category {
                if l != hai.hai().category() {
                    write!(f, "{}", l)?;
                }
            }
            write!(
                f,
                "{}{}{}",
                hai.to_prefix_str(),
                hai.hai().number(),
                hai.hai().to_dora_str()
            )?;
            last_category = Some(hai.hai().category());
        }
        if let Some(l) = last_category {
            write!(f, "{}", l)?;
        }
        Ok(())
    }
}

impl HaiVec {
    pub(crate) fn new(v: impl Into<Vec<HaiWithAttr>>) -> Self {
        Self(v.into())
    }
}

#[derive(Debug, Error)]
pub(crate) enum ParseError {
    #[error("number not found")]
    NumberNotFound,
    #[error("category not found at last")]
    CategoryNotFound,
    #[error("invalid char found: `{0}`")]
    InvalidChar(char),
    #[error("multiple prefix found: `{0}` and `{1}`")]
    MultiplePrefix(Prefix, Prefix),
    #[error("multiple dora sign found: `$`")]
    MultipleDora,
    #[error("multiple category found: `{0}` and `{1}`")]
    MultipleCategory(HaiCategory, HaiCategory),
    #[error(transparent)]
    NewHai(#[from] hai::NewError),
}

impl FromStr for HaiVec {
    type Err = ParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        use ParseError::*;

        let mut builders: Vec<Builder> = vec![];
        while !s.is_empty() {
            let mut builder = Builder::default();

            // prefix
            while let Some((prefix, rest)) = parse_prefix(s) {
                if let Some(old_prefix) = builder.prefix.replace(prefix) {
                    return Err(MultiplePrefix(old_prefix, prefix));
                }
                s = rest;
            }

            // number
            let mut chars = s.chars();
            let ch = chars.next().ok_or(NumberNotFound)?;
            let number = ch.to_digit(10).ok_or(InvalidChar(ch))?;
            assert!(builder.number.is_none());
            builder.number.replace(number as u8);
            s = chars.as_str();

            // red (dora)
            while let Some(rest) = s.strip_prefix('$') {
                if builder.red {
                    return Err(MultipleDora);
                }
                builder.red = true;
                s = rest;
            }

            // suffix
            while let Some((category, rest)) = parse_category(s) {
                if let Some(old_category) = builder.category.replace(category) {
                    return Err(MultipleCategory(old_category, category));
                }
                for b in builders.iter_mut().rev() {
                    match b.category {
                        Some(_) => break,
                        None => b.category = Some(category),
                    }
                }
                s = rest;
            }

            builders.push(builder);
        }

        Result::from_iter(builders.into_iter().map(Builder::build)).map(Self)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Prefix {
    Tacha(Tacha),
    Kakan,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Tacha(tacha) => write!(f, "{}", tacha),
            Prefix::Kakan => write!(f, "+"),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Builder {
    prefix: Option<Prefix>,
    number: Option<u8>,
    category: Option<HaiCategory>,
    red: bool,
    tacha: Option<Tacha>,
}

impl Builder {
    fn build(self) -> Result<HaiWithAttr, ParseError> {
        use HaiWithAttr::*;
        use ParseError::*;
        let hai = match (self.number, self.category) {
            (Some(number), Some(category)) => Hai::try_new(category, number, self.red)?,
            _ => return Err(CategoryNotFound),
        };

        let res = match self.prefix {
            None => FromTehai(hai),
            Some(Prefix::Tacha(p)) => FromTacha(p, hai),
            Some(Prefix::Kakan) => Kakan(hai),
        };
        Ok(res)
    }
}

fn parse_prefix(s: &str) -> Option<(Prefix, &str)> {
    for p in Tacha::into_enum_iter() {
        if let Some(rest) = s.strip_prefix(p.to_str()) {
            return Some((Prefix::Tacha(p), rest));
        }
    }
    if let Some(rest) = s.strip_prefix('+') {
        return Some((Prefix::Kakan, rest));
    }
    None
}

fn parse_category(s: &str) -> Option<(HaiCategory, &str)> {
    for c in HaiCategory::into_enum_iter() {
        if let Some(rest) = s.strip_prefix(c.to_str()) {
            return Some((c, rest));
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn parse() {
        use ParseError::*;
        fn ok(s: &str) -> String {
            HaiVec::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseError {
            HaiVec::from_str(s).unwrap_err()
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
        assert_eq!(ok("<1^2j>3+4j"), "<1^2>3+4j");

        assert_matches!(err("123p<"), NumberNotFound);
        assert_matches!(err("123p456"), CategoryNotFound);
        assert_matches!(err("123p&"), InvalidChar('&'));
        assert_matches!(err("123p0m"), NewHai(_));
        assert_matches!(err("<+3p"), MultiplePrefix(a, b) if h!("<+", a, b));
        assert_matches!(err("<<8m"), MultiplePrefix(a, b) if h!("<<", a, b));
        assert_matches!(err("123p8$$m"), MultipleDora);
    }
}
