use crate::{hai::Hai, hai_vec::HaiVec, hai_with_attr::HaiWithAttr, machi::Machi};
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mentsu(MentsuKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MentsuKind {
    /// 順子
    Shuntsu([Hai; 3]),
    /// 刻子
    Kotsu([Hai; 3]),
    /// 槓子
    Kantsu([Hai; 4]),
    /// 対子
    Toitsu([Hai; 2]),
}

impl Mentsu {
    pub(crate) fn shuntsu(hai: [Hai; 3]) -> Self {
        debug_assert!(hai[1].is_next_to(&hai[0]) && hai[2].is_next_to(&hai[1]));
        Self(MentsuKind::Shuntsu(hai))
    }

    pub(crate) fn kotsu(hai: [Hai; 3]) -> Self {
        debug_assert!(hai[0].is_same(&hai[1]) && hai[0].is_same(&hai[2]));
        Self(MentsuKind::Kotsu(hai))
    }

    pub(crate) fn kantsu(hai: [Hai; 4]) -> Self {
        debug_assert!(
            hai[0].is_same(&hai[1]) && hai[0].is_same(&hai[2]) && hai[0].is_same(&hai[3])
        );
        Self(MentsuKind::Kantsu(hai))
    }

    pub(crate) fn toitsu(hai: [Hai; 2]) -> Self {
        debug_assert!(hai[0].is_same(&hai[1]));
        Self(MentsuKind::Toitsu(hai))
    }

    fn to_hai_vec(&self) -> HaiVec {
        let t = HaiWithAttr::FromTehai;
        match self.0 {
            MentsuKind::Shuntsu([h0, h1, h2]) | MentsuKind::Kotsu([h0, h1, h2]) => {
                HaiVec::new([t(h0), t(h1), t(h2)])
            }
            MentsuKind::Kantsu([h0, h1, h2, h3]) => HaiVec::new([t(h0), t(h1), t(h2), t(h3)]),
            MentsuKind::Toitsu([h0, h1]) => HaiVec::new([t(h0), t(h1)]),
        }
    }

    pub(crate) fn to_machi(&self, agari: Hai) -> Option<Machi> {
        match self.0 {
            MentsuKind::Shuntsu([h0, h1, h2]) => {
                if h2.is_same(&agari) {
                    if h2.number() == 3 {
                        return Some(Machi::Penchan);
                    }
                    return Some(Machi::Ryanmen);
                }
                if h1.is_same(&agari) {
                    return Some(Machi::Kanchan);
                }
                if h0.is_same(&agari) {
                    if h0.number() == 7 {
                        return Some(Machi::Penchan);
                    }
                    return Some(Machi::Ryanmen);
                }
                None
            }
            MentsuKind::Kotsu([h0, ..]) => h0.is_same(&agari).then(|| Machi::Shanpon),
            MentsuKind::Kantsu(_) => None,
            MentsuKind::Toitsu([h0, _]) => h0.is_same(&agari).then(|| Machi::Tanki),
        }
    }

    pub(crate) fn compute_fu(&self, is_menzen: bool, bakaze: Hai, jikaze: Hai) -> u32 {
        match self.0 {
            MentsuKind::Shuntsu(_) => 0,
            MentsuKind::Kotsu([h0, ..]) => match (is_menzen, h0.is_yaochuhai()) {
                (false, false) => 2, // 中張牌 / 明刻
                (false, true) => 4,  // ヤオ九牌 / 明刻
                (true, false) => 4,  // 中張牌 / 暗刻
                (true, true) => 8,   // ヤオ九牌 / 暗刻
            },
            MentsuKind::Kantsu([h0, ..]) => match (is_menzen, h0.is_yaochuhai()) {
                (false, false) => 8, // 中張牌 / 明槓
                (false, true) => 16, // ヤオ九牌 / 明槓
                (true, false) => 16, // 中張牌 / 暗槓
                (true, true) => 32,  // ヤオ九牌 / 暗槓
            },
            MentsuKind::Toitsu([h0, ..]) => {
                if h0.is_sangenpai() {
                    2
                } else {
                    match (h0 == bakaze, h0 == jikaze) {
                        (true, true) => 4,                  // 連風牌
                        (false, true) | (true, false) => 2, // 自風牌 or 場風牌
                        (false, false) => 0,                // 他
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
impl Mentsu {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_js(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Mentsu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hai_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        furo::Furo, jun_tehai::JunTehai, kotsu_candidates::KotsuCandidates,
        shuntsu_candidates::ShuntsuCandidates, toitsu_candidates::ToitsuCandidates,
    };
    use std::str::FromStr;

    fn new_hai(s: &str) -> Hai {
        Hai::from_str(s).unwrap()
    }

    fn new_shuntsu(s: &str) -> Mentsu {
        let tehai = JunTehai::from_str(s).unwrap();
        let comb = ShuntsuCandidates::new(tehai.as_slice(), 0).collect::<Vec<_>>();
        assert_eq!(comb.len(), 1);
        comb[0].0
    }

    fn new_kotsu(s: &str) -> Mentsu {
        let tehai = JunTehai::from_str(s).unwrap();
        let comb = KotsuCandidates::new(tehai.as_slice(), 0).collect::<Vec<_>>();
        assert_eq!(comb.len(), 1);
        comb[0].0
    }

    fn new_kantsu(s: &str) -> Mentsu {
        Mentsu::from(Furo::from_str(s).unwrap())
    }

    fn new_toitsu(s: &str) -> Mentsu {
        let tehai = JunTehai::from_str(s).unwrap();
        let comb = ToitsuCandidates::new(tehai.as_slice(), 0, false).collect::<Vec<_>>();
        assert_eq!(comb.len(), 1);
        comb[0].0
    }

    #[test]
    fn to_machi() {
        fn shuntsu(s: &str, machi: &str) -> Option<Machi> {
            new_shuntsu(s).to_machi(new_hai(machi))
        }
        fn kotsu(s: &str, machi: &str) -> Option<Machi> {
            new_kotsu(s).to_machi(new_hai(machi))
        }
        fn toitsu(s: &str, machi: &str) -> Option<Machi> {
            new_toitsu(s).to_machi(new_hai(machi))
        }
        assert_eq!(shuntsu("234m", "3m"), Some(Machi::Kanchan));
        assert_eq!(shuntsu("123p", "3p"), Some(Machi::Penchan));
        assert_eq!(toitsu("77s", "7s"), Some(Machi::Tanki));
        assert_eq!(shuntsu("345p", "3p"), Some(Machi::Ryanmen));
        assert_eq!(shuntsu("456p", "6p"), Some(Machi::Ryanmen));
        assert_eq!(kotsu("666j", "6j"), Some(Machi::Shanpon));
        assert_eq!(kotsu("333s", "3s"), Some(Machi::Shanpon));
    }

    #[test]
    fn compute_fu() {
        fn shuntsu(s: &str) -> u32 {
            let bakaze = new_hai("1j");
            let jikaze = bakaze;
            let shuntsu = new_shuntsu(s);
            let menzen = shuntsu.compute_fu(true, bakaze, jikaze);
            let chi = shuntsu.compute_fu(false, bakaze, jikaze);
            assert_eq!(menzen, chi);
            menzen
        }
        fn kotsu(s: &str, is_menzen: bool) -> u32 {
            let bakaze = new_hai("1j");
            let jikaze = bakaze;
            new_kotsu(s).compute_fu(is_menzen, bakaze, jikaze)
        }
        fn kantsu(s: &str, is_menzen: bool) -> u32 {
            let bakaze = new_hai("1j");
            let jikaze = bakaze;
            new_kantsu(s).compute_fu(is_menzen, bakaze, jikaze)
        }
        fn toitsu(s: &str, bakaze: Hai, jikaze: Hai) -> u32 {
            new_toitsu(s).compute_fu(true, bakaze, jikaze)
        }

        let ton = new_hai("1j");
        let nan = new_hai("2j");
        let sha = new_hai("3j");
        let pe = new_hai("4j");

        // 面子の符 https://mj-king.net/tensu/tensu/2_01.html
        assert_eq!(shuntsu("123p"), 0); // 順子
        assert_eq!(kotsu("444m", false), 2); // 明刻 / 中張牌
        assert_eq!(kotsu("444m", true), 4); // 暗刻 / 中張牌
        assert_eq!(kotsu("777j", false), 4); // 明刻 / ヤオ九牌
        assert_eq!(kotsu("777j", true), 8); // 暗刻 / ヤオ九牌
        assert_eq!(toitsu("22j", nan, nan), 4); // 連風牌
        assert_eq!(toitsu("33j", nan, nan), 0); // オタ風
        assert_eq!(toitsu("66j", ton, ton), 2); // 三元牌 / 風牌
        assert_eq!(kantsu("2222s", false), 8); // 明槓 / 中張牌
        assert_eq!(kantsu("2222s", true), 16); // 暗槓 / 中張牌
        assert_eq!(kantsu("1111j", false), 16); // 明槓 / ヤオ九牌
        assert_eq!(kantsu("1111j", true), 32); // 暗槓 / ヤオ九牌

        // 順子は0符 https://mj-king.net/tensu/tensu/2_02.html
        for s in &["789m", "456p", "345s"] {
            assert_eq!(shuntsu(s), 0);
        }
        // 中張牌のポンは2符 https://mj-king.net/tensu/tensu/2_03.html
        for s in &["333p", "888s", "666m"] {
            assert_eq!(kotsu(s, false), 2);
        }
        // ヤオ九牌のポンは4符 https://mj-king.net/tensu/tensu/2_04.html
        for s in &["444j", "999p", "777j"] {
            assert_eq!(kotsu(s, false), 4);
        }
        // 中張牌の暗刻は4符 https://mj-king.net/tensu/tensu/2_05.html
        for s in &["333m", "666p", "888s"] {
            assert_eq!(kotsu(s, true), 4);
        }
        // ヤオ九牌の暗刻は8符 https://mj-king.net/tensu/tensu/2_06.html
        for s in &["444j", "999p", "777j"] {
            assert_eq!(kotsu(s, true), 8);
        }
        // 中張牌の明槓は8符 https://mj-king.net/tensu/tensu/2_07.html
        for s in &["3333m", "6666p", "8888s"] {
            assert_eq!(kantsu(s, false), 8);
        }
        // ヤオ九牌の明槓は16符 https://mj-king.net/tensu/tensu/2_08.html
        for s in &["4444j", "9999p", "7777j"] {
            assert_eq!(kantsu(s, false), 16);
        }
        // 中張牌の暗槓は16符 https://mj-king.net/tensu/tensu/2_09.html
        for s in &["3333m", "6666p", "8888s"] {
            assert_eq!(kantsu(s, true), 16);
        }
        // ヤオ九牌の暗槓は32符 https://mj-king.net/tensu/tensu/2_10.html
        for s in &["4444j", "9999p", "7777j"] {
            assert_eq!(kantsu(s, true), 32);
        }
        // 中張牌の雀頭は0符 https://mj-king.net/tensu/tensu/2_11.html
        for s in &["33m", "66p", "88s"] {
            assert_eq!(toitsu(s, ton, ton), 0);
        }
        // 一九牌とオタ風の雀頭は0符 https://mj-king.net/tensu/tensu/2_12.html
        for s in &["11s", "99p", "44j"] {
            assert_eq!(toitsu(s, ton, ton), 0);
        }
        // 三元牌、風牌の雀頭は2符 https://mj-king.net/tensu/tensu/2_13.html
        for s in &["55j", "66j", "77j"] {
            assert_eq!(toitsu(s, ton, nan), 2);
        }
        for (s, k0, k1) in &[
            ("11j", ton, nan),
            ("22j", nan, sha),
            ("33j", sha, pe),
            ("44j", pe, ton),
        ] {
            assert_eq!(toitsu(s, *k0, *k1), 2);
            assert_eq!(toitsu(s, *k1, *k0), 2);
        }
        // 連風牌の雀頭は4符 https://mj-king.net/tensu/tensu/2_14.html
        for (s, k) in &[("11j", ton), ("22j", nan), ("33j", sha), ("44j", pe)] {
            assert_eq!(toitsu(s, *k, *k), 4);
        }
    }
}
