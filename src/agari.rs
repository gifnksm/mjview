use crate::{
    agari_type::AgariType,
    env::Env,
    furo::FuroKind,
    hai_category::HaiCategory,
    machi::Machi,
    mentsu::{Mentsu, MentsuKind},
    tehai::Tehai,
    yaku::Yaku,
};
use std::{cmp::Ordering, fmt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Agari {
    tehai: Tehai,
    tehai_mentsu: Vec<Mentsu>,
    machi: Machi,
    machi_mentsu_index: usize,
    janto_mentsu_index: Option<usize>,
    /// 牌の数
    num_hai: usize,
    /// 萬子の数
    num_manzu: usize,
    /// 筒子の数
    num_pinzu: usize,
    /// 索子の数
    num_souzu: usize,
    /// 字牌の数
    num_jihai: usize,
    /// 暗順の数
    num_anshun: usize,
    /// 明順の数
    num_minshun: usize,
    /// 暗刻の数
    num_anko: usize,
    /// 明刻の数
    num_minko: usize,
    /// 暗槓の数
    num_ankan: usize,
    /// 明槓の数
    num_minkan: usize,
    /// 対子の数
    num_toitsu: usize,
    /// 単独牌の数
    num_single: usize,
}

impl fmt::Display for Agari {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, mentsu) in self.tehai_mentsu.iter().enumerate() {
            if idx == self.machi_mentsu_index {
                write!(f, "{{{}}}", mentsu)?;
            } else {
                write!(f, "{}", mentsu)?;
            }
            if idx + 1 < self.tehai_mentsu.len() {
                write!(f, ",")?;
            }
        }
        for furo in self.tehai.furo() {
            write!(f, " {}", furo)?;
        }
        Ok(())
    }
}

impl Agari {
    pub(crate) fn new(
        tehai: Tehai,
        tehai_mentsu: Vec<Mentsu>,
        machi: Machi,
        machi_mentsu_index: usize,
    ) -> Self {
        let is_tsumo = tehai.agari_hai().type_() == AgariType::Tsumo;
        let janto_mentsu_index = tehai_mentsu
            .iter()
            .position(|mentsu| matches!(mentsu.kind(), MentsuKind::Toitsu(..)));
        let num_hai = tehai.all_hai().count();
        let num_manzu = tehai
            .all_hai()
            .filter(|hai| hai.category() == HaiCategory::Manzu)
            .count();
        let num_souzu = tehai
            .all_hai()
            .filter(|hai| hai.category() == HaiCategory::Souzu)
            .count();
        let num_pinzu = tehai
            .all_hai()
            .filter(|hai| hai.category() == HaiCategory::Pinzu)
            .count();
        let num_jihai = tehai
            .all_hai()
            .filter(|hai| hai.category() == HaiCategory::Jihai)
            .count();
        let num_anshun = tehai_mentsu
            .iter()
            .enumerate()
            .filter(|(i, mentsu)| {
                (is_tsumo || *i != machi_mentsu_index)
                    && matches!(mentsu.kind(), MentsuKind::Shuntsu(..))
            })
            .count();
        let num_minshun = tehai
            .furo()
            .iter()
            .filter(|furo| matches!(furo.kind(), FuroKind::Chi { .. }))
            .count()
            + if !is_tsumo
                && matches!(
                    tehai_mentsu[machi_mentsu_index].kind(),
                    MentsuKind::Shuntsu(..)
                )
            {
                1
            } else {
                0
            };
        let num_anko = tehai_mentsu
            .iter()
            .enumerate()
            .filter(|(i, mentsu)| {
                (is_tsumo || *i != machi_mentsu_index)
                    && matches!(mentsu.kind(), MentsuKind::Kotsu(..))
            })
            .count();
        let num_minko = tehai
            .furo()
            .iter()
            .filter(|furo| matches!(furo.kind(), FuroKind::Pon { .. }))
            .count()
            + if !is_tsumo
                && matches!(
                    tehai_mentsu[machi_mentsu_index].kind(),
                    MentsuKind::Kotsu(..)
                )
            {
                1
            } else {
                0
            };
        let num_ankan = tehai
            .furo()
            .iter()
            .filter(|furo| matches!(furo.kind(), FuroKind::Ankan { .. }))
            .count();
        let num_minkan = tehai
            .furo()
            .iter()
            .filter(|furo| {
                matches!(
                    furo.kind(),
                    FuroKind::Kakan { .. } | FuroKind::Daiminkan { .. }
                )
            })
            .count();
        let num_toitsu = tehai_mentsu
            .iter()
            .filter(|mentsu| matches!(mentsu.kind(), MentsuKind::Toitsu(..)))
            .count();
        let num_single = tehai_mentsu
            .iter()
            .filter(|mentsu| matches!(mentsu.kind(), MentsuKind::Single(..)))
            .count();
        assert!(
            num_toitsu == 7
                || num_anshun
                    + num_minshun
                    + num_anko
                    + num_minko
                    + num_ankan
                    + num_minkan
                    + num_toitsu
                    == 5
                || (num_toitsu == 1 && num_single == 12)
        );
        Self {
            tehai,
            tehai_mentsu,
            machi,
            machi_mentsu_index,
            janto_mentsu_index,
            num_hai,
            num_manzu,
            num_pinzu,
            num_souzu,
            num_jihai,
            num_anshun,
            num_minshun,
            num_anko,
            num_minko,
            num_ankan,
            num_minkan,
            num_toitsu,
            num_single,
        }
    }

    pub(crate) fn tehai(&self) -> &Tehai {
        &self.tehai
    }

    pub(crate) fn tehai_mentsu(&self) -> &[Mentsu] {
        &self.tehai_mentsu
    }

    pub(crate) fn machi(&self) -> Machi {
        self.machi
    }

    pub(crate) fn machi_mentsu(&self) -> Mentsu {
        self.tehai_mentsu[self.machi_mentsu_index]
    }

    pub(crate) fn janto(&self) -> Option<Mentsu> {
        self.janto_mentsu_index
            .and_then(|idx| self.tehai_mentsu.get(idx).copied())
    }

    pub(crate) fn all_mentsu(&self) -> impl Iterator<Item = Mentsu> + '_ {
        self.tehai_mentsu()
            .iter()
            .copied()
            .chain(self.tehai().furo().iter().map(|furo| Mentsu::from(*furo)))
    }

    pub(crate) fn num_hai(&self) -> usize {
        self.num_hai
    }

    pub(crate) fn num_manzu(&self) -> usize {
        self.num_manzu
    }

    pub(crate) fn num_souzu(&self) -> usize {
        self.num_souzu
    }

    pub(crate) fn num_pinzu(&self) -> usize {
        self.num_pinzu
    }

    pub(crate) fn num_jihai(&self) -> usize {
        self.num_jihai
    }

    pub(crate) fn num_anshun(&self) -> usize {
        self.num_anshun
    }

    pub(crate) fn num_minshun(&self) -> usize {
        self.num_minshun
    }

    pub(crate) fn num_anko(&self) -> usize {
        self.num_anko
    }

    pub(crate) fn num_minko(&self) -> usize {
        self.num_minko
    }

    pub(crate) fn num_ankan(&self) -> usize {
        self.num_ankan
    }

    pub(crate) fn num_minkan(&self) -> usize {
        self.num_minkan
    }

    pub(crate) fn num_toitsu(&self) -> usize {
        self.num_toitsu
    }

    pub(crate) fn num_single(&self) -> usize {
        self.num_single
    }

    pub(crate) fn compute_fu(&self, env: &Env) -> u32 {
        let is_menzen = self.tehai.is_menzen();
        if self.tehai_mentsu.len() == 7 {
            // 七対子
            return 25;
        }

        let is_ron = self.tehai.agari_hai().type_() == AgariType::Ron;
        let tehai_mentsu = self
            .tehai_mentsu
            .iter()
            .enumerate()
            .map(|(idx, mentsu)| {
                let is_menzen = !is_ron || self.machi_mentsu_index != idx;
                mentsu.compute_fu(is_menzen, env)
            })
            .sum::<u32>();
        let furo = self
            .tehai
            .furo()
            .iter()
            .map(|furo| furo.compute_fu(env))
            .sum::<u32>();
        let machi = self.machi.compute_fu();

        if tehai_mentsu + furo + machi == 0 {
            // 平和形
            if is_menzen {
                if !is_ron {
                    // ツモ平和
                    return 20;
                }
            } else {
                // 食い平和
                return 30;
            }
        }

        const FUTEI: u32 = 20; // 副底
        let menzen_kafu = if is_ron && is_menzen { 10 } else { 0 };
        let tsumofu = if is_ron { 0 } else { 2 };
        let total = FUTEI + tehai_mentsu + furo + machi + menzen_kafu + tsumofu;
        (total + 9) / 10 * 10
    }

    pub(crate) fn judge_yaku(&self, env: &Env) -> Yaku {
        Yaku::new(self, env)
    }
}

#[wasm_bindgen]
impl Agari {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string_js(&self) -> String {
        self.to_string()
    }

    #[wasm_bindgen(js_name = "judgeYaku")]
    pub fn judge_yaku_js(&self, env: &Env) -> Yaku {
        self.judge_yaku(env)
    }

    #[wasm_bindgen(js_name = compare)]
    pub fn compare_js(&self, other: &Agari) -> i32 {
        match self.cmp(other) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{hai::Hai, tehai::Tehai};
    use std::str::FromStr;

    #[test]
    fn compute_fu() {
        let ton = Hai::from_str("1j").unwrap();
        let nan = Hai::from_str("2j").unwrap();
        fn comp(s: &str, bakaze: Hai, jikaze: Hai) -> u32 {
            let env = Env::new_empty(bakaze, jikaze);
            let tehai = Tehai::from_str(s).unwrap();
            let comb = tehai.to_agari_combinations();
            assert_eq!(comb.len(), 1);
            comb[0].compute_fu(&env)
        }

        // https://ja.wikipedia.org/wiki/%E9%BA%BB%E9%9B%80%E3%81%AE%E5%BE%97%E7%82%B9%E8%A8%88%E7%AE%97#%E7%AC%A6%E3%81%AE%E8%A8%88%E7%AE%97
        assert_eq!(comp("113344m5566s77p1j !1j", ton, ton), 25); // 七対子
        assert_eq!(comp("55s12345678m234p ?9m", ton, ton), 30); // 平和 (ロン)
        assert_eq!(comp("55s12345678m234p !9m", ton, ton), 20); // 平和 (ツモ)
        assert_eq!(comp("55s12378m234p <456m ?9m", ton, ton), 30); // 食い平和 (ロン)
        assert_eq!(comp("55s12378m234p <456m !9m", ton, ton), 30); // 食い平和 (ツモ)
        assert_eq!(comp("77j12p666j >999m ^888+8p !3p", ton, ton), 50);
        assert_eq!(comp("234s1177j 9999p 1111m ?7j", ton, ton), 110);
        assert_eq!(comp("678s22j11m 6666j 1111p ?1m", nan, nan), 110);
        assert_eq!(comp("234p1j 9999m 9999s 1111s ?1j", ton, ton), 140);
        assert_eq!(comp("7j 4444j 1111m 9999p 1111s ?7j", ton, ton), 170);
        assert_eq!(comp("55677m23466p567s !6m", ton, ton), 30);
        assert_eq!(comp("123789p3467899s !2s", ton, nan), 20);
        assert_eq!(comp("456789p34578s22j ?6s", ton, ton), 30);
        assert_eq!(comp("123p3j <978s <645s <312s ?3j", ton, nan), 30);
        assert_eq!(comp("223344s77p5m11j77j !5m", nan, ton), 25);
        assert_eq!(comp("333m77788p111s77j ?7j", nan, ton), 50);
        assert_eq!(comp("1299p666777j <555j ?3p", ton, nan), 50);

        // アガリの形につく点数 https://mj-king.net/tensu/tensu/1_05.html
        // 副底: 20符、二索暗刻: 4符、嵌張待ち: 2符 + α
        assert_eq!(comp("11p223344m222s46p ?5p", ton, ton), 40); // 門前ロン (+10符 = 36符 -> 40符)
        assert_eq!(comp("11p223344m222s46p !5p", ton, ton), 30); // 門前ツモ (+2符 = 28符 -> 30符)
        assert_eq!(comp("11p234m222s46p <234m ?5p", ton, ton), 30); // 鳴きのロン (+ 0符 = 26符 -> 30符)
        assert_eq!(comp("11p234m222s46p <234m !5p", ton, ton), 30); // 鳴きのツモ (+2符 = 28符 -> 30符)

        // 門前ロンは10符をプラス https://mj-king.net/tensu/tensu/1_06.html
        // 副底: 20符、白暗刻: 8符、辺張待ち: 2符 + 門前ロン: 10符 = 40符 -> 40符
        assert_eq!(comp("66s666j12345689m ?7m", ton, ton), 40);
        // 門前ツモは2符をプラス https://mj-king.net/tensu/tensu/1_07.html
        // 副底: 20符、白暗刻: 8符、辺張待ち: 2符 + ツモ: 2符 = 32符 -> 40符
        assert_eq!(comp("66s666j12345689m !7m", ton, ton), 40);
        // 鳴きのロンは0符 https://mj-king.net/tensu/tensu/1_08.html
        // 副底: 20符、白暗刻: 8符、辺張待ち: 2符 + 鳴きのロン: 0符 = 30符 -> 30符
        assert_eq!(comp("66s666j45689m <213m ?7m", ton, ton), 30);
        // 鳴きのツモは2符 https://mj-king.net/tensu/tensu/1_09.html
        // 副底: 20符、白暗刻: 8符、辺張待ち: 2符 + ツモ: 2符 = 32符 -> 40符
        assert_eq!(comp("66s666j45689m <213m !7m", ton, ton), 40);
        // 門前ロン10符、ツモ2符 https://mj-king.net/tensu/tensu/1_10.html
        // 副底: 20符、八索暗刻: 4符、嵌張待ち: 2符 + α
        assert_eq!(comp("888s233445p3588m !4m", ton, ton), 30); // ツモ (+2符 = 28符 -> 30符)
        assert_eq!(comp("888s233445p3588m ?4m", ton, ton), 40); // 門前ロン (+10符 = 36符 -> 40符)

        // 20符のあがり https://mj-king.net/tensu/tensu/4_03.html
        assert_eq!(comp("23456p678m12399s !1p", ton, ton), 20);
        // 30符のあがり https://mj-king.net/tensu/tensu/4_04.html
        assert_eq!(comp("1245566799m ^666j !3m", ton, ton), 30);
        assert_eq!(comp("1245566799m ^666j ?3m", ton, ton), 30);
        // 40符のあがり https://mj-king.net/tensu/tensu/4_05.html
        // ※画像が50符と逆になっている
        assert_eq!(comp("77755j123m79p <312p ?8p", ton, ton), 40);
        // 50符のあがり https://mj-king.net/tensu/tensu/4_06.html
        // ※画像が40符と逆になっている
        assert_eq!(comp("22p11233s999m777s ?2s", ton, ton), 50);
        // 60符のあがり https://mj-king.net/tensu/tensu/4_07.html
        assert_eq!(comp("111666j55s88m >1111m ?5s", ton, ton), 60);
        // 70符のあがり https://mj-king.net/tensu/tensu/4_08.html
        // 画像は80符のもののため省略
        // 80符のあがり https://mj-king.net/tensu/tensu/4_09.html
        assert_eq!(comp("444p88m77p ^4444j 1111s ?7p", ton, ton), 80);
        // 平和ロンはかならず30符になる https://mj-king.net/tensu/tensu/4_12.html
        assert_eq!(comp("44m123456789s34p ?2p", ton, ton), 30);
        // 平和ツモはかならず20符になる https://mj-king.net/tensu/tensu/4_13.html
        assert_eq!(comp("23444m234s234p78s !9s", ton, ton), 20);
    }
}
