/// 待ち
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Machi {
    /// 両面待ち
    Ryanmen,
    /// 双碰待ち
    Shanpon,
    /// 嵌張待ち
    Kanchan,
    /// 辺張待ち
    Penchan,
    /// 単騎待ち
    Tanki,
}

impl Machi {
    pub(crate) fn compute_fu(&self) -> u32 {
        match self {
            Machi::Ryanmen | Machi::Shanpon => 0,
            Machi::Kanchan | Machi::Penchan | Machi::Tanki => 2,
        }
    }
}
