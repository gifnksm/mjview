use crate::hai::{self, Hai, HaiCategory};
use enum_iterator::IntoEnumIterator;
use std::{fmt, str::FromStr};
use thiserror::Error;

/// プレイヤー
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoEnumIterator)]
pub(crate) enum Player {
    /// 上家
    Kamicha,
    /// 対面
    Toimen,
    /// 下家
    Shimocha,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tacha_mark())
    }
}

impl Player {
    fn tacha_mark(&self) -> char {
        use Player::*;
        match self {
            Kamicha => '<',
            Toimen => '^',
            Shimocha => '>',
        }
    }
}

/// 副露
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Furo {
    /// チー
    Chi {
        from_tehai: [Hai; 2], //< 手牌にあった牌
        from_kamicha: Hai,    //< 上家から取得した牌
    },
    /// ポン
    Pon {
        from_tehai: [Hai; 2], //< 手牌にあった牌
        from_tacha: Hai,      //< 他家から取得した牌
        player: Player,       //< 牌の取得元のプレイヤー
    },
    /// 加槓
    Kakan {
        from_tehai: [Hai; 2], //< ポン時に手牌にあった牌
        from_tacha: Hai,      //< ポン時に他家から取得した牌
        player: Player,       //< ポン時の牌の取得元のプレイヤー
        added: Hai,           //< 加槓で追加した牌
    },
    /// 大明槓
    Daiminkan {
        from_tehai: [Hai; 3], //< 手牌にあった牌
        from_tacha: Hai,      //< 他家から取得した牌
        player: Player,       //< 牌の取得元のプレイヤー
    },
    /// 暗槓
    Ankan {
        from_tehai: [Hai; 4], //< 手牌にあった牌
    },
}

impl fmt::Display for Furo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Furo::*;
        macro_rules! hai {
            ($n:expr) => {
                format_args!("{}{}", $n.number(), if $n.red() { "$" } else { "" })
            };
            ($n:expr, $($rest:expr),+ $(,)?) => {
                format_args!("{}{}", hai!($n), hai!($($rest),+))
            }
        }
        match self {
            // <312m
            Chi {
                from_tehai: [t0, t1],
                from_kamicha: k0,
            } => write!(
                f,
                "{}{}{}",
                Player::Kamicha.tacha_mark(),
                hai!(k0, t0, t1),
                k0.category(),
            ),
            // ^333m
            Pon {
                from_tehai: [te0, te1],
                from_tacha: ta,
                player,
            } => write!(
                f,
                "{}{}{}",
                player.tacha_mark(),
                hai!(ta, te0, te1),
                ta.category()
            ),
            // >333+3m
            Kakan {
                from_tehai: [te0, te1],
                from_tacha: ta,
                player,
                added: a,
            } => write!(
                f,
                "{}{}+{}{}",
                player.tacha_mark(),
                hai!(ta, te0, te1),
                hai!(a),
                ta.category()
            ),
            // ^3333m
            Daiminkan {
                from_tehai: [te0, te1, te2],
                from_tacha: ta,
                player,
            } => write!(
                f,
                "{}{}{}",
                player.tacha_mark(),
                hai!(ta, te0, te1, te2),
                ta.category()
            ),
            // 3333m
            Ankan {
                from_tehai: [t0, t1, t2, t3],
            } => write!(f, "{}{}", hai!(t0, t1, t2, t3), t0.category()),
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum ParseFuroError {
    #[error("number not found")]
    NumberNotFound,
    #[error("category not found at last")]
    CategoryNotFound,
    #[error("multiple categories found")]
    MultipleCategories,
    #[error("multiple hai from tacha found")]
    MultipleTacha,
    #[error("multiple kakan hai found")]
    MultipleKakan,
    #[error("invalid char found: `{0}`")]
    InvalidChar(char),
    #[error("menzen chi found")]
    MenzenChi,
    #[error("menzen pon found")]
    MenzenPon,
    #[error("chi not from kamicha: `{0}` from `{1}`")]
    ChiNotFromKamicha(Hai, Player),
    #[error("chi with kakan: `{0}`")]
    ChiWithKakan(Hai),
    #[error("pon with kakan: `{0}`")]
    PonWithKakan(Hai),
    #[error("ankan with kakan: `{0}`")]
    AnkanWithKakan(Hai),
    #[error("invalid tehai combination for furo")]
    InvalidTehaiCombination,
    #[error(transparent)]
    NewHai(#[from] hai::NewError),
}

#[derive(Debug, Clone, Copy)]
enum Prefix {
    Player(Player),
    Kakan,
}

fn parse_prefix(s: &str) -> Option<(Prefix, &str)> {
    for p in Player::into_enum_iter() {
        if let Some(rest) = s.strip_prefix(p.tacha_mark()) {
            return Some((Prefix::Player(p), rest));
        }
    }
    if let Some(rest) = s.strip_prefix('+') {
        return Some((Prefix::Kakan, rest));
    }
    None
}

fn parse_category(s: &str) -> Option<(HaiCategory, &str)> {
    for c in HaiCategory::into_enum_iter() {
        if let Some(rest) = s.strip_prefix(c.as_char()) {
            return Some((c, rest));
        }
    }
    None
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HaiAttribute {
    FromTehai,
    FromTacha(Player),
    Kakan,
}

#[derive(Debug, Clone, Default)]
struct HaiBuilder {
    prefix: Option<Prefix>,
    number: Option<u8>,
    category: Option<HaiCategory>,
    red: bool,
    player: Option<Player>,
}

impl HaiBuilder {
    fn build(self) -> Result<(Hai, HaiAttribute), ParseFuroError> {
        use HaiAttribute::*;
        let hai = match (self.number, self.category) {
            (Some(number), Some(category)) => Hai::try_new(category, number, self.red)?,
            _ => return Err(ParseFuroError::CategoryNotFound),
        };

        let res = match self.prefix {
            None => (hai, FromTehai),
            Some(Prefix::Player(p)) => (hai, FromTacha(p)),
            Some(Prefix::Kakan) => (hai, Kakan),
        };
        Ok(res)
    }
}

impl FromStr for Furo {
    type Err = ParseFuroError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        use ParseFuroError::*;

        let mut builders: Vec<HaiBuilder> = vec![];
        while !s.is_empty() {
            let mut builder = HaiBuilder::default();
            while let Some((prefix, rest)) = parse_prefix(s) {
                builder.prefix = Some(prefix);
                s = rest;
            }

            // number
            let mut chars = s.chars();
            let ch = chars.next().ok_or(ParseFuroError::NumberNotFound)?;
            let number = ch.to_digit(10).ok_or(ParseFuroError::InvalidChar(ch))?;
            builder.number = Some(number as u8);
            s = chars.as_str();

            // red (dora)
            while let Some(rest) = s.strip_prefix('$') {
                builder.red = true;
                s = rest;
            }

            // suffix
            while let Some((category, rest)) = parse_category(s) {
                if builder.category.is_none() {
                    builder.category = Some(category);
                    for b in builders.iter_mut().rev() {
                        match b.category {
                            Some(bc) if bc != category => return Err(MultipleCategories),
                            Some(_) => break,
                            None => b.category = Some(category),
                        }
                    }
                }
                s = rest;
            }

            builders.push(builder);
        }

        let hai_pairs = builders.into_iter().map(HaiBuilder::build);

        let mut all_hai = vec![];
        let mut from_tehai = vec![];
        let mut from_tacha = None;
        let mut kakan = None;

        for res in hai_pairs {
            let (hai, attr) = res?;
            all_hai.push(hai);
            match attr {
                HaiAttribute::FromTehai => from_tehai.push(hai),
                HaiAttribute::FromTacha(p) => {
                    if from_tacha.is_some() {
                        return Err(MultipleTacha);
                    }
                    from_tacha = Some((p, hai));
                }
                HaiAttribute::Kakan => {
                    if kakan.is_some() {
                        return Err(MultipleKakan);
                    }
                    kakan = Some(hai);
                }
            }
        }

        all_hai.sort();
        from_tehai.sort();

        let res = match &all_hai[..] {
            // チー
            [h0, h1, h2]
                if h0.category() != HaiCategory::Jihai
                    && h0.number() + 1 == h1.number()
                    && h1.number() + 1 == h2.number() =>
            {
                let (player, from_tacha) = from_tacha.ok_or(MenzenChi)?;
                if player != Player::Kamicha {
                    return Err(ChiNotFromKamicha(from_tacha, player));
                }
                if let Some(hai) = kakan {
                    return Err(ChiWithKakan(hai));
                }
                assert_eq!(from_tehai.len(), 2);
                Furo::Chi {
                    from_tehai: [from_tehai[0], from_tehai[1]],
                    from_kamicha: from_tacha,
                }
            }
            // ポン
            [h0, h1, h2] if h0.number() == h1.number() && h1.number() == h2.number() => {
                let (player, from_tacha) = from_tacha.ok_or(MenzenPon)?;
                if let Some(hai) = kakan {
                    return Err(PonWithKakan(hai));
                }
                assert_eq!(from_tehai.len(), 2);
                Furo::Pon {
                    from_tehai: [from_tehai[0], from_tehai[1]],
                    from_tacha,
                    player,
                }
            }
            // 槓
            [h0, h1, h2, h3]
                if h0.number() == h1.number()
                    && h1.number() == h2.number()
                    && h2.number() == h3.number() =>
            {
                match (from_tacha, kakan) {
                    (Some((player, from_tacha)), Some(kakan)) => {
                        assert_eq!(from_tehai.len(), 2);
                        Furo::Kakan {
                            from_tehai: [from_tehai[0], from_tehai[1]],
                            from_tacha,
                            player,
                            added: kakan,
                        }
                    }
                    (Some((player, from_tacha)), None) => {
                        assert_eq!(from_tehai.len(), 3);
                        Furo::Daiminkan {
                            from_tehai: [from_tehai[0], from_tehai[1], from_tehai[2]],
                            from_tacha,
                            player,
                        }
                    }
                    (None, None) => {
                        assert_eq!(from_tehai.len(), 4);
                        Furo::Ankan {
                            from_tehai: [
                                from_tehai[0],
                                from_tehai[1],
                                from_tehai[2],
                                from_tehai[3],
                            ],
                        }
                    }
                    (None, Some(kakan)) => return Err(AnkanWithKakan(kakan)),
                }
            }
            _ => return Err(ParseFuroError::InvalidTehaiCombination),
        };
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn parse_furo() {
        use ParseFuroError::*;
        fn ok(s: &str) -> String {
            Furo::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseFuroError {
            Furo::from_str(s).unwrap_err()
        }

        // チー
        assert_eq!(ok("1<23p"), "<213p");
        assert_eq!(ok("4<5$p3p"), "<5$34p");
        assert_matches!(err("456p"), MenzenChi);
        assert_matches!(err("45^6p"),
            ChiNotFromKamicha(hai, player) if hai.to_string() == "6p" && player == Player::Toimen);
        assert_matches!(err("4<5+6m"), ChiWithKakan(hai) if hai.to_string() == "6m");

        // ポン
        assert_eq!(ok("3^33j"), "^333j");
        assert_eq!(ok("5^5$5m"), "^5$55m");
        assert_matches!(err("333p"), MenzenPon);
        assert_matches!(err("5+5^5j"), PonWithKakan(hai) if hai.to_string() == "5j");

        // 加槓
        assert_eq!(ok("+1<111j"), "<111+1j");

        // 大明槓
        assert_eq!(ok("5>5$55m"), ">5$555m");

        // 暗槓
        assert_eq!(ok("3333p"), "3333p");
        assert_matches!(err("33+33p"), AnkanWithKakan(hai) if hai.to_string() == "3p");
    }
}
