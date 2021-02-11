use crate::{
    hai::Hai, hai_category::HaiCategory, hai_image::HaiImage, hai_vec::HaiVec,
    hai_with_attr::HaiWithAttr, mentsu::Mentsu, tacha::Tacha,
};
use std::{fmt, str::FromStr};
use thiserror::Error;
use wasm_bindgen::prelude::*;

/// 副露
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Furo(FuroKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FuroKind {
    /// チー
    Chi {
        from_tehai: [Hai; 2], //< 手牌にあった牌
        from_kamicha: Hai,    //< 上家から取得した牌
    },
    /// ポン
    Pon {
        from_tehai: [Hai; 2], //< 手牌にあった牌
        from_tacha: Hai,      //< 他家から取得した牌
        tacha: Tacha,         //< 牌の取得元のプレイヤー
    },
    /// 加槓
    Kakan {
        from_tehai: [Hai; 2], //< ポン時に手牌にあった牌
        from_tacha: Hai,      //< ポン時に他家から取得した牌
        tacha: Tacha,         //< ポン時の牌の取得元のプレイヤー
        added: Hai,           //< 加槓で追加した牌
    },
    /// 大明槓
    Daiminkan {
        from_tehai: [Hai; 3], //< 手牌にあった牌
        from_tacha: Hai,      //< 他家から取得した牌
        tacha: Tacha,         //< 牌の取得元のプレイヤー
    },
    /// 暗槓
    Ankan {
        from_tehai: [Hai; 4], //< 手牌にあった牌
    },
}

impl fmt::Display for Furo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_vec())
    }
}

impl Furo {
    pub(crate) fn to_vec(&self) -> HaiVec {
        use HaiWithAttr::*;
        match self.0 {
            FuroKind::Chi {
                from_tehai: [t0, t1],
                from_kamicha: k0,
            } => HaiVec::new([FromTacha(Tacha::Kamicha, k0), FromTehai(t0), FromTehai(t1)]),
            FuroKind::Pon {
                from_tehai: [te0, te1],
                from_tacha: ta,
                tacha,
            } => HaiVec::new([FromTacha(tacha, ta), FromTehai(te0), FromTehai(te1)]),
            FuroKind::Kakan {
                from_tehai: [te0, te1],
                from_tacha: ta,
                tacha,
                added: a,
            } => HaiVec::new([
                FromTacha(tacha, ta),
                FromTehai(te0),
                FromTehai(te1),
                Kakan(a),
            ]),
            FuroKind::Daiminkan {
                from_tehai: [te0, te1, te2],
                from_tacha: ta,
                tacha,
            } => HaiVec::new([
                FromTacha(tacha, ta),
                FromTehai(te0),
                FromTehai(te1),
                FromTehai(te2),
            ]),
            FuroKind::Ankan {
                from_tehai: [t0, t1, t2, t3],
            } => HaiVec::new([FromTehai(t0), FromTehai(t1), FromTehai(t2), FromTehai(t3)]),
        }
    }

    fn to_image(&self) -> Vec<HaiImage> {
        use HaiImage as H;
        match self.0 {
            FuroKind::Chi {
                from_tehai: [t0, t1],
                from_kamicha: k0,
            } => vec![H::sideways(k0), H::normal(t0), H::normal(t1)],
            FuroKind::Pon {
                from_tehai: [te0, te1],
                from_tacha: ta,
                tacha,
            } => {
                let ta = H::sideways(ta);
                let (te0, te1) = (H::normal(te0), H::normal(te1));
                match tacha {
                    Tacha::Kamicha => vec![ta, te0, te1],
                    Tacha::Toimen => vec![te0, ta, te1],
                    Tacha::Shimocha => vec![te0, te1, ta],
                }
            }
            FuroKind::Kakan {
                from_tehai: [te0, te1],
                from_tacha: ta,
                tacha,
                added: a,
            } => {
                let s = H::stack(ta, a);
                let (te0, te1) = (H::normal(te0), H::normal(te1));
                match tacha {
                    Tacha::Kamicha => vec![s, te0, te1],
                    Tacha::Toimen => vec![te0, s, te1],
                    Tacha::Shimocha => vec![te0, te1, s],
                }
            }
            FuroKind::Daiminkan {
                from_tehai: [te0, te1, te2],
                from_tacha: ta,
                tacha,
            } => {
                let ta = H::sideways(ta);
                let (te0, te1, te2) = (H::normal(te0), H::normal(te1), H::normal(te2));
                match tacha {
                    Tacha::Kamicha => vec![ta, te0, te1, te2],
                    Tacha::Toimen => vec![te0, ta, te1, te2],
                    Tacha::Shimocha => vec![te0, te1, te2, ta],
                }
            }
            FuroKind::Ankan {
                from_tehai: [t0, t1, t2, t3],
            } => vec![H::hidden(t0), H::normal(t1), H::normal(t2), H::hidden(t3)],
        }
    }
}

#[wasm_bindgen]
impl Furo {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_js(&self) -> String {
        self.to_string()
    }

    #[wasm_bindgen(js_name = "toImage")]
    pub fn to_image_js(&self) -> Box<[JsValue]> {
        self.to_image().into_iter().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "fromStr")]
    pub fn from_str_js(s: &str) -> Result<Furo, JsValue> {
        let res = Self::from_str(s).map_err(|e| e.to_string())?;
        Ok(res)
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] ParseErrorKind);

#[derive(Debug, Error)]
enum ParseErrorKind {
    #[error("multiple categories found: `{0}` and `{1}`")]
    MultipleCategories(HaiWithAttr, HaiWithAttr),
    #[error("multiple hai from tacha found: `{}` and `{}`", HaiWithAttr::FromTacha(*.0, *.1), HaiWithAttr::FromTacha(*.2, *.3))]
    MultipleTacha(Tacha, Hai, Tacha, Hai),
    #[error("multiple kakan hai found: `{}` and `{}`", HaiWithAttr::Kakan(*.0), HaiWithAttr::Kakan(*.1))]
    MultipleKakan(Hai, Hai),
    #[error("menzen chi found: `{}`", HaiVec::new([HaiWithAttr::FromTehai(*.0), HaiWithAttr::FromTehai(*.1), HaiWithAttr::FromTehai(*.2)]))]
    MenzenChi(Hai, Hai, Hai),
    #[error("menzen pon found: `{}`", HaiVec::new([HaiWithAttr::FromTehai(*.0), HaiWithAttr::FromTehai(*.1), HaiWithAttr::FromTehai(*.2)]))]
    MenzenPon(Hai, Hai, Hai),
    #[error("chi not from kamicha: `{}", HaiWithAttr::FromTacha(*.0, *.1))]
    ChiNotFromKamicha(Tacha, Hai),
    #[error("chi with kakan: `{}`", HaiWithAttr::Kakan(*.0))]
    ChiWithKakan(Hai),
    #[error("pon with kakan: `{}`", HaiWithAttr::Kakan(*.0))]
    PonWithKakan(Hai),
    #[error("ankan with kakan: `{}`", HaiWithAttr::Kakan(*.0))]
    AnkanWithKakan(Hai),
    #[error("invalid tehai combination for furo: `{0}`")]
    InvalidCombination(HaiVec),
    #[error("invalid hai found: `{0}`")]
    InvalidHai(HaiWithAttr),
    #[error(transparent)]
    HaiVec(#[from] <HaiVec as FromStr>::Err),
}

impl FromStr for Furo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseErrorKind as E;

        let hai_vec = HaiVec::from_str(s).map_err(E::from)?;

        let mut all_hai = vec![];
        let mut from_tehai = vec![];
        let mut from_tacha = None;
        let mut kakan = None;

        for attr in &hai_vec.0 {
            all_hai.push(*attr.hai());
            if all_hai[0].category() != attr.hai().category() {
                return Err(E::MultipleCategories(hai_vec.0[0], *attr).into());
            }
            match *attr {
                HaiWithAttr::FromTehai(hai) => from_tehai.push(hai),
                HaiWithAttr::FromTacha(tacha, hai) => {
                    if let Some((old_tacha, old_hai)) = from_tacha.replace((tacha, hai)) {
                        return Err(E::MultipleTacha(old_tacha, old_hai, tacha, hai).into());
                    }
                }
                HaiWithAttr::Kakan(hai) => {
                    if let Some(old_hai) = kakan.replace(hai) {
                        return Err(E::MultipleKakan(old_hai, hai).into());
                    }
                }
                _ => return Err(E::InvalidHai(*attr).into()),
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
                let (tacha, from_tacha) = from_tacha.ok_or_else(|| E::MenzenChi(*h0, *h1, *h2))?;
                if tacha != Tacha::Kamicha {
                    return Err(E::ChiNotFromKamicha(tacha, from_tacha).into());
                }
                if let Some(hai) = kakan {
                    return Err(E::ChiWithKakan(hai).into());
                }
                assert_eq!(from_tehai.len(), 2);
                FuroKind::Chi {
                    from_tehai: [from_tehai[0], from_tehai[1]],
                    from_kamicha: from_tacha,
                }
            }
            // ポン
            [h0, h1, h2] if h0.number() == h1.number() && h1.number() == h2.number() => {
                let (tacha, from_tacha) = from_tacha.ok_or_else(|| E::MenzenPon(*h0, *h1, *h2))?;
                if let Some(hai) = kakan {
                    return Err(E::PonWithKakan(hai).into());
                }
                assert_eq!(from_tehai.len(), 2);
                FuroKind::Pon {
                    from_tehai: [from_tehai[0], from_tehai[1]],
                    from_tacha,
                    tacha,
                }
            }
            // 槓
            [h0, h1, h2, h3]
                if h0.number() == h1.number()
                    && h1.number() == h2.number()
                    && h2.number() == h3.number() =>
            {
                match (from_tacha, kakan) {
                    (Some((tacha, from_tacha)), Some(kakan)) => {
                        assert_eq!(from_tehai.len(), 2);
                        FuroKind::Kakan {
                            from_tehai: [from_tehai[0], from_tehai[1]],
                            from_tacha,
                            tacha,
                            added: kakan,
                        }
                    }
                    (Some((tacha, from_tacha)), None) => {
                        assert_eq!(from_tehai.len(), 3);
                        FuroKind::Daiminkan {
                            from_tehai: [from_tehai[0], from_tehai[1], from_tehai[2]],
                            from_tacha,
                            tacha,
                        }
                    }
                    (None, None) => {
                        assert_eq!(from_tehai.len(), 4);
                        FuroKind::Ankan {
                            from_tehai: [
                                from_tehai[0],
                                from_tehai[1],
                                from_tehai[2],
                                from_tehai[3],
                            ],
                        }
                    }
                    (None, Some(kakan)) => return Err(E::AnkanWithKakan(kakan).into()),
                }
            }
            _ => return Err(E::InvalidCombination(hai_vec).into()),
        };
        Ok(Furo(res))
    }
}

impl From<Furo> for Mentsu {
    fn from(furo: Furo) -> Self {
        match furo.0 {
            FuroKind::Chi {
                from_tehai: [te0, te1],
                from_kamicha: ka,
            } => {
                let mut v = [te0, te1, ka];
                v.sort();
                Mentsu::shuntsu(v)
            }
            FuroKind::Pon {
                from_tehai: [te0, te1],
                from_tacha: ta,
                tacha: _,
            } => {
                let mut v = [te0, te1, ta];
                v.sort();
                Mentsu::kotsu(v)
            }
            FuroKind::Kakan {
                from_tehai: [te0, te1],
                from_tacha: ta,
                tacha: _,
                added: a,
            } => {
                let mut v = [te0, te1, ta, a];
                v.sort();
                Mentsu::kantsu(v)
            }
            FuroKind::Daiminkan {
                from_tehai: [te0, te1, te2],
                from_tacha: ta,
                tacha: _,
            } => {
                let mut v = [te0, te1, te2, ta];
                v.sort();
                Mentsu::kantsu(v)
            }
            FuroKind::Ankan { from_tehai: mut v } => {
                v.sort();
                Mentsu::kantsu(v)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn parse_furo() {
        use ParseErrorKind::*;
        fn ok(s: &str) -> String {
            Furo::from_str(s).unwrap().to_string()
        }
        fn err(s: &str) -> ParseErrorKind {
            Furo::from_str(s).unwrap_err().0
        }
        macro_rules! h {
            ($expected:expr, $($expr:expr),*) => {
                {
                    assert_eq!($expected, [$($expr.to_string()),*].join(""));
                    true
                }
            }
        }

        // チー
        assert_eq!(ok("1<23p"), "<213p");
        assert_eq!(ok("4<5$p3p"), "<5$34p");
        assert_matches!(err("456p"), MenzenChi(a, b, c) if h!("4p5p6p", a, b, c));
        assert_matches!(err("45^6p"), ChiNotFromKamicha(tacha, hai) if h!("^6p", tacha, hai));
        assert_matches!(err("4<5+6m"), ChiWithKakan(hai) if h!("6m", hai));

        // ポン
        assert_eq!(ok("3^33j"), "^333j");
        assert_eq!(ok("5^5$5m"), "^5$55m");
        assert_matches!(err("333p"), MenzenPon(a, b, c) if h!("3p3p3p", a, b, c));
        assert_matches!(err("5+5^5j"), PonWithKakan(hai) if h!("5j", hai));

        // 加槓
        assert_eq!(ok("+1<111j"), "<111+1j");

        // 大明槓
        assert_eq!(ok("5>5$55m"), ">5$555m");

        // 暗槓
        assert_eq!(ok("3333p"), "3333p");
        assert_matches!(err("33+33p"), AnkanWithKakan(hai) if h!("3p", hai));

        // その他
        assert_matches!(err("<3s3s3m"), MultipleCategories(a, b) if h!("<3s3m", a, b));
        assert_matches!(err("<3>45p"), MultipleTacha(a, b, c, d) if h!("<3p>4p", a, b, c, d));
        assert_matches!(err("<3+4+5p"), MultipleKakan(a, b) if h!("4p5p", a, b));
        assert_matches!(err("<346p"), InvalidCombination(a) if h!("<346p", a));
        assert_matches!(err("<1!23p"), InvalidHai(hai) if h!("!2p", hai));
        assert_matches!(err("<012p"), HaiVec(_));
    }
}
