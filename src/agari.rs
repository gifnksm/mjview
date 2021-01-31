use enum_iterator::IntoEnumIterator;
use std::fmt;

/// あがりの種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoEnumIterator)]
pub(crate) enum Agari {
    Tsumo,
    Ron,
}

impl Agari {
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            Self::Tsumo => "!",
            Self::Ron => "?",
        }
    }
}

impl fmt::Display for Agari {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
