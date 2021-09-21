use enum_iterator::IntoEnumIterator;
use std::fmt;

/// 他家 (自家以外のプレイヤー)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoEnumIterator)]
pub(crate) enum Tacha {
    /// 上家
    Kamicha,
    /// 対面
    Toimen,
    /// 下家
    Shimocha,
}

impl fmt::Display for Tacha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Tacha {
    pub(crate) fn to_str(self) -> &'static str {
        use Tacha::*;
        match self {
            Kamicha => "<",
            Toimen => "^",
            Shimocha => ">",
        }
    }
}
