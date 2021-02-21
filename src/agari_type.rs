use enum_iterator::IntoEnumIterator;
use std::fmt;

/// あがりの種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoEnumIterator)]
pub(crate) enum AgariType {
    Tsumo,
    Ron,
}

impl AgariType {
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            Self::Tsumo => "!",
            Self::Ron => "?",
        }
    }

    pub(crate) fn is_agari_str(s: &str) -> bool {
        Self::into_enum_iter().any(|a| s.starts_with(a.to_str()))
    }
}

impl fmt::Display for AgariType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
