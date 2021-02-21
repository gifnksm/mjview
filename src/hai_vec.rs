use crate::{
    hai_builder::{Error as ParseError, HaiBuilder},
    hai_with_attr::HaiWithAttr,
};
use std::{fmt, iter::FromIterator, str::FromStr};

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

impl FromStr for HaiVec {
    type Err = ParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut builders: Vec<HaiBuilder> = vec![];
        while !s.is_empty() {
            let mut builder = HaiBuilder::new(true);
            s = builder.eat_str(s)?;
            if let Some(category) = builder.category() {
                for b in builders.iter_mut().rev() {
                    if b.set_category(category).is_err() {
                        break;
                    }
                }
            }
            builders.push(builder);
        }

        Result::from_iter(builders.into_iter().map(HaiBuilder::build)).map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hai_builder::ErrorKind as ParseErrorKind;
    use assert_matches::assert_matches;

    #[test]
    fn parse() {
        use ParseErrorKind::*;
        fn ok(s: &str) -> String {
            HaiVec::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseErrorKind {
            HaiVec::from_str(s).unwrap_err().into()
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
