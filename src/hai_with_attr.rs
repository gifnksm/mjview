use crate::{agari::Agari, hai::Hai, tacha::Tacha};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum HaiWithAttr {
    FromTehai(Hai),
    FromTacha(Tacha, Hai),
    Kakan(Hai),
    Agari(Agari, Hai),
}

impl HaiWithAttr {
    pub(crate) fn hai(&self) -> &Hai {
        match self {
            Self::FromTehai(hai) => hai,
            Self::FromTacha(_, hai) => hai,
            Self::Kakan(hai) => hai,
            Self::Agari(_, hai) => hai,
        }
    }

    pub(crate) fn to_prefix_str(&self) -> &str {
        match self {
            Self::FromTehai(_) => "",
            Self::FromTacha(tacha, _) => tacha.to_str(),
            Self::Kakan(_) => "+",
            Self::Agari(agari, _) => agari.to_str(),
        }
    }
}

impl fmt::Display for HaiWithAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.to_prefix_str(), self.hai())
    }
}
