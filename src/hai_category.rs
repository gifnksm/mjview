use enum_iterator::IntoEnumIterator;
use std::fmt;

/// 牌の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoEnumIterator)]
pub(crate) enum HaiCategory {
    Manzu, //< 萬子
    Pinzu, //< 筒子
    Souzu, //< 索子
    Jihai, //< 字牌
}

impl HaiCategory {
    pub(crate) fn to_str(&self) -> &'static str {
        use HaiCategory::*;
        match self {
            Manzu => "m",
            Pinzu => "p",
            Souzu => "s",
            Jihai => "j",
        }
    }
}

impl fmt::Display for HaiCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
