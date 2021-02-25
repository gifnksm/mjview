use crate::{
    agari::Agari,
    agari_type::AgariType,
    env::Env,
    hai::Hai,
    hai_category::HaiCategory,
    machi::Machi,
    mentsu::MentsuKind,
    rank::{Rank, RankKind},
};
use js_sys::Array;
use std::{borrow::Cow, iter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Yaku {
    name: Cow<'static, str>,
    point: u32,
    fu: u32,
    rank: Rank,
    detail: Vec<(&'static str, Rank)>,
}

impl Yaku {
    pub(crate) fn new(agari: &Agari, env: &Env) -> Self {
        let fu = agari.compute_fu(env);
        let (rank, detail) = judge(agari, env);
        let (name, point) = compute_point(agari, env, fu, rank);
        Self {
            name,
            point,
            fu,
            rank,
            detail,
        }
    }
}

#[wasm_bindgen]
impl Yaku {
    #[wasm_bindgen(getter = name)]
    pub fn name_js(&self) -> String {
        self.name.to_string()
    }

    #[wasm_bindgen(getter = point)]
    pub fn point_js(&self) -> u32 {
        self.point
    }

    #[wasm_bindgen(getter = fu)]
    pub fn fu_js(&self) -> u32 {
        self.fu
    }

    #[wasm_bindgen(getter = rank)]
    pub fn rank_js(&self) -> Rank {
        self.rank
    }

    #[wasm_bindgen(getter = detail)]
    pub fn detail_js(&self) -> Array {
        self.detail
            .iter()
            .map(|(name, rank)| {
                iter::once(JsValue::from(*name))
                    .chain(iter::once(JsValue::from(*rank)))
                    .collect::<Array>()
            })
            .collect()
    }
}

fn judge(agari: &Agari, env: &Env) -> (Rank, Vec<(&'static str, Rank)>) {
    if let Some((total, list)) = judge_list(agari, env, &YAKUMAN_LIST, Rank::new_yakuman) {
        return (Rank::new_yakuman(total), list);
    }

    if let Some((mut total, mut list)) = judge_list(agari, env, &YAKU_LIST, Rank::new_fan) {
        if let Some((dora_fan, mut dora_list)) =
            judge_list(agari, env, &ADDITIONAL_LIST, Rank::new_fan)
        {
            total += dora_fan;
            list.append(&mut dora_list);
        }
        return (Rank::new_fan(total), list);
    }

    (Rank::new_fan(0), vec![])
}

fn judge_list(
    agari: &Agari,
    env: &Env,
    list: &[JudgeFn],
    gen: impl Fn(u32) -> Rank,
) -> Option<(u32, Vec<(&'static str, Rank)>)> {
    let mut total_rank = 0;
    let mut res = vec![];
    for f in list {
        if let Some((name, rank)) = f(agari, env) {
            total_rank += rank;
            res.push((name, gen(rank)));
        }
    }
    (!res.is_empty()).then(|| (total_rank, res))
}

fn compute_point(agari: &Agari, env: &Env, fu: u32, rank: Rank) -> (Cow<'static, str>, u32) {
    let (name, base_point) = compute_base_point(fu, rank);
    let is_oya = env.jikaze.number() == 1;
    let is_ron = agari.tehai().agari_hai().type_() == AgariType::Ron;
    fn round(point: u32) -> u32 {
        (point + 99) / 100 * 100
    }
    let point = match (is_oya, is_ron) {
        (true, true) => round(base_point * 6),
        (false, true) => round(base_point * 4),
        (true, false) => round(base_point * 2) * 3,
        (false, false) => round(base_point * 2) + round(base_point) * 2,
    };

    (name, point)
}

fn compute_base_point(fu: u32, rank: Rank) -> (Cow<'static, str>, u32) {
    match rank.kind() {
        RankKind::Fan(fan) if *fan <= 5 => {
            let base_point = fu * 2u32.pow(fan + 2);
            if base_point <= 2000 {
                ("".into(), base_point)
            } else {
                ("満貫".into(), 2000)
            }
        }
        RankKind::Fan(fan) if *fan <= 7 => ("跳満".into(), 3000),
        RankKind::Fan(fan) if *fan <= 10 => ("倍満".into(), 4000),
        RankKind::Fan(fan) if *fan <= 12 => ("三倍満".into(), 6000),
        RankKind::Fan(_) => ("数え役満".into(), 8000),
        RankKind::Yakuman(1) => ("役満".into(), 8000),
        RankKind::Yakuman(n) => (format!("{}倍役満", n).into(), n * 8000),
    }
}

type JudgeFn = fn(agari: &Agari, env: &Env) -> Option<(&'static str, u32)>;

const YAKU_LIST: &[JudgeFn] = &[
    richi, ippatsu, tsumo, tanyao, pinfu, ipeko, bakaze, jikaze, haku, hatsu, chun, rinshan,
    haitei, sanshoku, ittsu, chanta, chitoi, toitoi, sananko, honro, sandoko, sankantsu, shosan,
    daburi, honitsu, junchan, ryanpeko, chinitsu,
];

const YAKUMAN_LIST: &[JudgeFn] = &[
    kokushi, suanko, daisangen, tsuiso, shosushi, daisushi, ryuiso, chinro, sukantsu, churen,
    tenho, chiho,
];

const ADDITIONAL_LIST: &[JudgeFn] = &[dora, uradora, akadora];

fn richi(_agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.richi.then(|| ("立直", 1))
}

fn ippatsu(_agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.ippatsu.then(|| ("一発", 1))
}

fn tsumo(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.tehai().is_menzen() && agari.tehai().agari_hai().type_() == AgariType::Tsumo)
        .then(|| ("門前清自摸和", 1))
}

fn tanyao(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    agari
        .tehai()
        .all_hai()
        .all(|hai| !hai.is_yaochuhai())
        .then(|| ("断么九", 1))
}

fn pinfu(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    (agari.tehai().is_menzen()
        && agari.machi() == Machi::Ryanmen
        && agari.num_anshun() + agari.num_minshun() == 4
        && agari.janto()?.compute_fu(true, env) == 0)
        .then(|| ("平和", 1))
}

fn ipeko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.tehai().is_menzen() && num_peko(agari) == 1).then(|| ("一盃口", 1))
}

fn bakaze(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    yakuhai(agari, |hai| env.bakaze.is_same(&hai)).then(|| ("役牌: 場風牌", 1))
}

fn jikaze(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    yakuhai(agari, |hai| env.jikaze.is_same(&hai)).then(|| ("役牌: 自風牌", 1))
}

fn haku(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    yakuhai(agari, |hai| {
        hai.number() == 5 && hai.category() == HaiCategory::Jihai
    })
    .then(|| ("役牌: 白", 1))
}

fn hatsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    yakuhai(agari, |hai| {
        hai.number() == 6 && hai.category() == HaiCategory::Jihai
    })
    .then(|| ("役牌: 發", 1))
}

fn chun(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    yakuhai(agari, |hai| {
        hai.number() == 7 && hai.category() == HaiCategory::Jihai
    })
    .then(|| ("役牌: 中", 1))
}

fn rinshan(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.rinshan.then(|| {
        if agari.tehai().agari_hai().type_() == AgariType::Tsumo {
            ("嶺上開花", 1)
        } else {
            ("搶槓", 1)
        }
    })
}

fn haitei(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.haitei.then(|| {
        if agari.tehai().agari_hai().type_() == AgariType::Tsumo {
            ("海底撈月", 1)
        } else {
            ("河底撈魚", 1)
        }
    })
}

fn sanshoku(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    let shuntsu_heads = agari
        .all_mentsu()
        .filter(|mentsu| matches!(mentsu.kind(), MentsuKind::Shuntsu(..)))
        .map(|mentsu| mentsu.head());
    let mut head_mask: Vec<(u8, u8)> = vec![];
    for head in shuntsu_heads {
        let mask = match head.category() {
            HaiCategory::Manzu => 0b001,
            HaiCategory::Pinzu => 0b010,
            HaiCategory::Souzu => 0b100,
            HaiCategory::Jihai => unreachable!(),
        };
        if let Some(idx) = head_mask.iter().position(|h| head.number() == h.0) {
            head_mask[idx].1 |= mask;
        } else {
            head_mask.push((head.number(), mask));
        }
    }
    head_mask
        .iter()
        .any(|h| h.1 == 0b111)
        .then(|| ("三色同順", kuisagari(agari, 2)))
}

fn ittsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    let mut manzu = 0;
    let mut souzu = 0;
    let mut pinzu = 0;
    let shuntsu_heads = agari
        .all_mentsu()
        .filter(|mentsu| matches!(mentsu.kind(), MentsuKind::Shuntsu(..)))
        .map(|mentsu| mentsu.head());
    for head in shuntsu_heads {
        if head.number() % 3 == 1 {
            let dest = match head.category() {
                HaiCategory::Manzu => &mut manzu,
                HaiCategory::Pinzu => &mut pinzu,
                HaiCategory::Souzu => &mut souzu,
                HaiCategory::Jihai => unreachable!(),
            };
            *dest |= 1 << (head.number() / 3);
        }
    }
    (manzu == 0b111 || souzu == 0b111 || pinzu == 0b111).then(|| ("一気通貫", kuisagari(agari, 2)))
}

fn chanta(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari
        .all_mentsu()
        .any(|mentsu| matches!(mentsu.kind(), MentsuKind::Shuntsu(..)))
        && agari.num_jihai() > 0
        && agari
            .all_mentsu()
            .all(|mentsu| mentsu.iter().any(|hai| hai.is_yaochuhai())))
    .then(|| ("混全帯么九", kuisagari(agari, 2)))
}

fn chitoi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_toitsu() == 7).then(|| ("七対子", 2))
}

fn toitoi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_anko() + agari.num_minko() + agari.num_ankan() + agari.num_minkan() == 4)
        .then(|| ("対対和", 2))
}

fn sananko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_anko() + agari.num_ankan() == 3).then(|| ("三暗刻", 2))
}

fn honro(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_jihai() > 0 && agari.tehai().all_hai().all(|hai| hai.is_yaochuhai()))
        .then(|| ("混老頭", 2))
}

fn sandoko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    let kotsu_heads = agari
        .all_mentsu()
        .filter(|mentsu| {
            matches!(
                mentsu.kind(),
                MentsuKind::Kotsu(..) | MentsuKind::Kantsu(..)
            )
        })
        .map(|mentsu| mentsu.head());
    let mut head_mask: Vec<(u8, u8)> = vec![];
    for head in kotsu_heads {
        let mask = match head.category() {
            HaiCategory::Manzu => 0b001,
            HaiCategory::Pinzu => 0b010,
            HaiCategory::Souzu => 0b100,
            HaiCategory::Jihai => continue,
        };
        if let Some(idx) = head_mask.iter().position(|h| head.number() == h.0) {
            head_mask[idx].1 |= mask;
        } else {
            head_mask.push((head.number(), mask));
        }
    }
    head_mask
        .iter()
        .any(|h| h.1 == 0b111)
        .then(|| ("三色同刻", 2))
}

fn sankantsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_ankan() + agari.num_minkan() == 3).then(|| ("三槓子", 2))
}

fn shosan(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.janto()?.head().is_sangenpai()
        && agari
            .all_mentsu()
            .filter(|mentsu| mentsu.head().is_sangenpai())
            .filter(|mentsu| {
                matches!(
                    mentsu.kind(),
                    MentsuKind::Kotsu(..) | MentsuKind::Kantsu(..)
                )
            })
            .count()
            == 2)
        .then(|| ("小三元", 2))
}

fn daburi(_agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.daburi.then(|| ("ダブル立直", 2))
}

fn honitsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_jihai() > 0
        && ((agari.num_jihai() + agari.num_manzu() == agari.num_hai())
            || (agari.num_jihai() + agari.num_souzu() == agari.num_hai())
            || (agari.num_jihai() + agari.num_pinzu() == agari.num_hai())))
    .then(|| ("混一色", kuisagari(agari, 3)))
}

fn junchan(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari
        .all_mentsu()
        .any(|mentsu| matches!(mentsu.kind(), MentsuKind::Shuntsu(..)))
        && agari.num_jihai() == 0
        && agari
            .all_mentsu()
            .all(|mentsu| mentsu.iter().any(|hai| hai.is_yaochuhai())))
    .then(|| ("純全帯么九", kuisagari(agari, 3)))
}

fn ryanpeko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.tehai().is_menzen() && num_peko(agari) == 2).then(|| ("二盃口", 3))
}

fn chinitsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_manzu() == agari.num_hai()
        || agari.num_pinzu() == agari.num_hai()
        || agari.num_souzu() == agari.num_hai())
    .then(|| ("清一色", kuisagari(agari, 6)))
}

fn kokushi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_toitsu() == 1 && agari.num_single() == 12).then(|| {
        if let MentsuKind::Toitsu(..) = agari.machi_mentsu().kind() {
            ("国士無双十三面待ち", 2)
        } else {
            ("国士無双", 1)
        }
    })
}

fn suanko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_anko() + agari.num_ankan() == 4).then(|| {
        if let MentsuKind::Toitsu(..) = agari.machi_mentsu().kind() {
            ("四暗刻単騎", 2)
        } else {
            ("四暗刻", 1)
        }
    })
}

fn daisangen(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari
        .all_mentsu()
        .filter(|mentsu| mentsu.head().is_sangenpai())
        .filter(|mentsu| {
            matches!(
                mentsu.kind(),
                MentsuKind::Kotsu(..) | MentsuKind::Kantsu(..)
            )
        })
        .count()
        == 3)
        .then(|| ("大三元", 1))
}

fn tsuiso(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_jihai() == agari.num_hai()).then(|| ("字一色", 1))
}

fn shosushi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.janto()?.head().is_kazehai()
        && agari
            .all_mentsu()
            .filter(|mentsu| mentsu.head().is_kazehai())
            .filter(|mentsu| {
                matches!(
                    mentsu.kind(),
                    MentsuKind::Kotsu(..) | MentsuKind::Kantsu(..)
                )
            })
            .count()
            == 3)
        .then(|| ("小四喜", 1))
}

fn daisushi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari
        .all_mentsu()
        .filter(|mentsu| mentsu.head().is_kazehai())
        .filter(|mentsu| {
            matches!(
                mentsu.kind(),
                MentsuKind::Kotsu(..) | MentsuKind::Kantsu(..)
            )
        })
        .count()
        == 4)
        .then(|| ("大四喜", 2))
}

fn ryuiso(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    agari
        .tehai()
        .all_hai()
        .all(|hai| match (hai.category(), hai.number()) {
            (HaiCategory::Souzu, n) if n == 2 || n == 3 || n == 4 || n == 6 || n == 8 => true,
            (HaiCategory::Jihai, 6) => true,
            _ => false,
        })
        .then(|| ("緑一色", 1))
}

fn chinro(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_jihai() == 0 && agari.tehai().all_hai().all(|hai| hai.is_yaochuhai()))
        .then(|| ("清老頭", 1))
}

fn sukantsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_ankan() + agari.num_minkan() == 4).then(|| ("四槓子", 1))
}

fn churen(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    if !agari.tehai().is_menzen()
        || (agari.num_manzu() != agari.num_hai()
            && agari.num_souzu() != agari.num_hai()
            && agari.num_pinzu() != agari.num_hai())
    {
        return None;
    }

    let expected = [3, 1, 1, 1, 1, 1, 1, 1, 3];
    let mut count = [0u8; 9];
    let mut extra = None;
    for hai in agari.tehai().all_hai() {
        let idx = (hai.number() - 1) as usize;
        count[idx] += 1;
        if count[idx] > expected[idx] + 1 {
            return None;
        }
        if count[idx] > expected[idx] {
            if extra.is_some() {
                return None;
            }
            extra = Some(hai);
        }
    }
    assert!(extra.is_some());
    if extra.unwrap().is_same(&agari.tehai().agari_hai().hai()) {
        Some(("純正九蓮宝燈", 2))
    } else {
        Some(("九蓮宝燈", 1))
    }
}

fn tenho(_agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.tenho.then(|| ("天和", 1))
}

fn chiho(_agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.chiho.then(|| ("地和", 1))
}

fn dora(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    let count = agari
        .tehai()
        .all_hai()
        .map(|hai| env.dora.iter().filter(|dora| hai.is_next_to(dora)).count())
        .sum::<usize>();
    (count > 0).then(|| ("ドラ", count as u32))
}

fn uradora(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    if !env.richi && !env.daburi {
        return None;
    }
    let count = agari
        .tehai()
        .all_hai()
        .map(|hai| {
            env.uradora
                .iter()
                .filter(|dora| hai.is_next_to(dora))
                .count()
        })
        .sum::<usize>();
    (count > 0).then(|| ("裏ドラ", count as u32))
}

fn akadora(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    let count = agari.tehai().all_hai().filter(|hai| hai.akadora()).count();
    (count > 0).then(|| ("赤ドラ", count as u32))
}

fn kuisagari(agari: &Agari, fan: u32) -> u32 {
    if agari.tehai().is_menzen() {
        fan
    } else {
        fan - 1
    }
}

fn num_peko(agari: &Agari) -> usize {
    let shuntsu_heads = agari
        .all_mentsu()
        .filter(|mentsu| matches!(mentsu.kind(), MentsuKind::Shuntsu(..)))
        .map(|mentsu| mentsu.head());
    let mut head_count: Vec<(Hai, usize)> = vec![];
    for head in shuntsu_heads {
        if let Some(idx) = head_count.iter().position(|h| head.is_same(&h.0)) {
            head_count[idx].1 += 1;
        } else {
            head_count.push((head, 1));
        }
    }
    // 同一順子4組の場合、二盃口に数える
    head_count.iter().filter(|(_, c)| *c > 1).count()
        + head_count.iter().filter(|(_, c)| *c > 3).count()
}

fn yakuhai(agari: &Agari, f: impl Fn(Hai) -> bool) -> bool {
    agari
        .all_mentsu()
        .filter(|mentsu| {
            matches!(
                mentsu.kind(),
                MentsuKind::Kotsu(..) | MentsuKind::Kantsu(..)
            )
        })
        .map(|mentsu| mentsu.head())
        .any(f)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{rank::RankKind, tehai::Tehai};
    use std::str::FromStr;

    fn yaku(s: &str, env: &Env) -> String {
        let tehai = Tehai::from_str(s).unwrap();
        let comb = tehai.to_agari_combinations();
        comb.into_iter()
            .map(|agari| {
                let s = judge(&agari, env)
                    .1
                    .into_iter()
                    .map(|(name, fan)| match fan.kind() {
                        RankKind::Fan(fan) => format!("{}:{}", name, fan),
                        RankKind::Yakuman(yakuman) => format!("{}:!{}", name, yakuman),
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                format!("[{}]", s)
            })
            .collect()
    }

    #[test]
    fn rich() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.richi = true;
        assert_eq!(yaku("344556m24678s66j ?3s", &env), "[立直:1]");
    }

    #[test]
    fn ippatsu() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.richi = true;
        env.ippatsu = true;
        assert_eq!(yaku("1112345m345s123p ?6m", &env), "[立直:1,一発:1,平和:1]");
    }

    #[test]
    fn tsumo() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        assert_eq!(yaku("345567m1113368s !7s", &env), "[門前清自摸和:1]");
        assert_eq!(
            yaku("345m345p3344588s !5s", &env),
            "[門前清自摸和:1,断么九:1,平和:1,一盃口:1,三色同順:2]",
        );
    }

    #[test]
    fn tanyao() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 門前のケース
        assert_eq!(yaku("22555m678s23477p ?2m", &env), "[断么九:1]");
        // 片あがりのケース
        assert_eq!(yaku("22555m678s23777p ?4p", &env), "[断么九:1]");
        // 副露したケース/三色同順との複合
        assert_eq!(
            yaku("22567m88s <657p <657s !2m", &env),
            "[断么九:1,三色同順:1]",
        );
        // 対対和との複合
        assert_eq!(
            yaku("22555m88s ^777p ^222p ?8s", &env),
            "[断么九:1,対対和:2]",
        );
        // 清一色との複合
        assert_eq!(
            yaku("2267888m ^333m <657m ?2m", &env),
            "[断么九:1,清一色:5]"
        );
    }

    #[test]
    fn pinfu() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("2j").unwrap());
        // 平和になる配姿
        assert_eq!(yaku("345567m234p3378s ?6s", &env), "[断么九:1,平和:1]");
        assert_eq!(yaku("345567m234p3378s ?9s", &env), "[平和:1]");
        // 平和にならない配姿-1
        assert_eq!(yaku("345567m234p3368s ?7s", &env), "[断么九:1]");
        // 平和にならない配姿-2
        assert_eq!(yaku("345567m234p2345s ?2s", &env), "[断么九:1]");
        assert_eq!(yaku("345567m234p2345s ?5s", &env), "[断么九:1]");
        // 平和になる待ちとならない待ちがある配姿
        assert_eq!(yaku("345567m123p3334s ?2s", &env), "[平和:1]");
        assert_eq!(yaku("345567m123p3334s ?5s", &env), "[平和:1]");
        assert_eq!(yaku("345567m123p3334s ?4s", &env), "[]");
        // 雀頭が役牌の場合は平和にならない
        assert_eq!(yaku("345567m234p22j78s ?6s", &env), "[]");
        // 平和になる配姿
        assert_eq!(yaku("34567m234p33789s ?2m", &env), "[平和:1]");
        assert_eq!(yaku("34567m234p33789s ?5m", &env), "[平和:1][平和:1]");
        assert_eq!(yaku("34567m234p33789s ?8m", &env), "[平和:1]");
        // 平和になる配姿
        assert_eq!(
            yaku("1123344m234789s ?2m", &env),
            "[一盃口:1][平和:1,一盃口:1]",
        );
        assert_eq!(yaku("1123344m234789s ?5m", &env), "[平和:1]");
        // 平和に取らない方が得点が高くなる配姿
        assert_eq!(
            yaku("2234455m234s234p ?3m", &env),
            "[断么九:1,一盃口:1,三色同順:2][断么九:1,平和:1,一盃口:1]",
        );
        assert_eq!(yaku("2234455m234s234p ?6m", &env), "[断么九:1,平和:1]");
    }

    #[test]
    fn ipeko() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 嵌張待ちのケース
        assert_eq!(yaku("33455m111p33789s ?4m", &env), "[一盃口:1]");
        // 高目で一盃口になるケース
        assert_eq!(yaku("33445m234p33789s ?2m", &env), "[平和:1]");
        assert_eq!(yaku("33445m234p33789s ?5m", &env), "[平和:1,一盃口:1]");
        // どちらであがっても一盃口になるケース
        assert_eq!(yaku("23334445m33789s ?2m", &env), "[平和:1,一盃口:1]");
        assert_eq!(yaku("23334445m33789s ?5m", &env), "[平和:1,一盃口:1]");
        // 出来合いのケース
        assert_eq!(yaku("334455m234p33s22j ?3s", &env), "[一盃口:1]");
        assert_eq!(yaku("334455m234p33s22j ?2j", &env), "[一盃口:1]");
        // 多面双碰の一部になるケース
        assert_eq!(yaku("33445566m234p22j ?3m", &env), "[一盃口:1]");
        assert_eq!(yaku("33445566m234p22j ?6m", &env), "[一盃口:1]");
        assert_eq!(yaku("33445566m234p22j ?2j", &env), "[一盃口:1][一盃口:1]");
        assert_eq!(
            yaku("3344556677m234p ?3m", &env),
            "[断么九:1,一盃口:1][断么九:1,一盃口:1]",
        );
        assert_eq!(yaku("3344556677m234p ?4m", &env), "[断么九:1,一盃口:1]");
        assert_eq!(yaku("3344556677m234p ?6m", &env), "[断么九:1,一盃口:1]");
        assert_eq!(
            yaku("3344556677m234p ?7m", &env),
            "[断么九:1,一盃口:1][断么九:1,一盃口:1]",
        );
        // 一盃口を構成する2面子と同一の順子がもう1つできているケース (一盃口子とは取らないケース)
        assert_eq!(
            yaku("333444555m34p33s ?2p", &env),
            "[断么九:1,平和:1,一盃口:1][断么九:1,三暗刻:2]",
        );
        // 一盃口にはならないケース
        assert_eq!(yaku("12233m3377p2277j ?1m", &env), "[七対子:2]");
    }

    #[test]
    fn yakuhai() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 門前のケース
        assert_eq!(
            yaku("345567m3378s111j ?6s", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1]",
        );
        // 副露したケース
        assert_eq!(yaku("345567m3378s ^777j ?9s", &env), "[役牌: 中:1]");
        assert_eq!(yaku("567m3378s777j <345m ?9s", &env), "[役牌: 中:1]");
        // 役牌同士の複合
        assert_eq!(
            yaku("345m111j777j3378s ?6s", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,役牌: 中:1]",
        );
    }

    #[test]
    fn sanshoku() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 出来合いだが三色のみの配姿
        assert_eq!(yaku("567m567p22567s99s ?9s", &env), "[三色同順:2]");
        // 出来合いのタンピン三色
        assert_eq!(
            yaku("234m23467p23488s ?5p", &env),
            "[断么九:1,平和:1,三色同順:2]",
        );
        // 高目と安目でできる役が違う配姿
        assert_eq!(yaku("33445m234p234s55j ?2m", &env), "[三色同順:2]");
        assert_eq!(yaku("33445m234p234s55j ?5m", &env), "[一盃口:1]");
        // 出来合いの純全帯么九三色
        assert_eq!(
            yaku("11123m123s12389p ?7p", &env),
            "[三色同順:2,純全帯么九:3]",
        );
        // 副露したケース
        assert_eq!(yaku("234m24789s77j <324p ?3s", &env), "[三色同順:1]");
    }

    #[test]
    fn ittsu() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 一気通貫が確定しているが待ちが狭いケース
        assert_eq!(yaku("12456789m33789s ?3m", &env), "[一気通貫:2]");
        // 待ちは広いが一気通貫が確定しないケース
        assert_eq!(yaku("23456789m33789s ?1m", &env), "[平和:1,一気通貫:2]");
        assert_eq!(yaku("23456789m33789s ?4m", &env), "[平和:1][平和:1]");
        assert_eq!(yaku("23456789m33789s ?7m", &env), "[平和:1][]");
        // 出来合いのケース
        assert_eq!(yaku("123456789m4478s ?9s", &env), "[平和:1,一気通貫:2]");
        // 副露したケース
        assert_eq!(yaku("456789m3378s <213m ?9s", &env), "[一気通貫:1]");
        // 清一色との複合
        assert_eq!(yaku("1122345678999m ?1m", &env), "[九蓮宝燈:!1]");
        assert_eq!(yaku("1122345678999m ?2m", &env), "[清一色:6]");
        assert_eq!(
            yaku("1122345678999m ?3m", &env),
            "[一盃口:1,一気通貫:2,清一色:6]",
        );

        assert_eq!(yaku("1234445678999m ?3m", &env), "[清一色:6][清一色:6]");
        assert_eq!(yaku("1234445678999m ?6m", &env), "[清一色:6][清一色:6]");
        assert_eq!(
            yaku("1234445678999m ?9m", &env),
            "[一気通貫:2,清一色:6][一気通貫:2,清一色:6]",
        );
        assert_eq!(
            yaku("1234445678999m ?4m", &env),
            "[一気通貫:2,清一色:6][一気通貫:2,清一色:6]",
        );
        assert_eq!(yaku("1234445678999m ?7m", &env), "[清一色:6][清一色:6]");
        assert_eq!(yaku("1234445678999m ?5m", &env), "[清一色:6]");
        assert_eq!(yaku("1234445678999m ?8m", &env), "[清一色:6]");
    }

    #[test]
    fn chanta() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        assert_eq!(yaku("123999m789p33j12s ?3s", &env), "[混全帯么九:2]");
        assert_eq!(
            yaku("123m123789p23s22j ?1s", &env),
            "[平和:1,三色同順:2,混全帯么九:2]"
        );
        assert_eq!(yaku("123m123789p23s22j ?4s", &env), "[平和:1]");
        assert_eq!(yaku("99m789p13s ^333j <123m ?2s", &env), "[混全帯么九:1]",);
    }

    #[test]
    fn chitoi() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 七対子のみのケース
        assert_eq!(yaku("115599m2233p8s22j ?8s", &env), "[七対子:2]");
        // 他の役が複合しているケース
        assert_eq!(yaku("22334466m55332j ?2j", &env), "[七対子:2,混一色:3]");
        // 一般的には七対子と認められないケース
        assert!(yaku("334455m2233p888s ?8s", &env).is_empty());
        // 高点法により七対子には取られない配姿
        assert_eq!(
            yaku("112233m55667s11p ?7s", &env),
            "[平和:1,二盃口:3][七対子:2]",
        );
    }

    #[test]
    fn toitoi() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 副露したケース
        assert_eq!(yaku("55888m33j ^111p ^222s ?5m", &env), "[対対和:2]");
        assert_eq!(yaku("55888m33j ^111p ^222s ?3j", &env), "[対対和:2]");
        // 門前のケース
        assert_eq!(yaku("55888m33j222s111p ?5m", &env), "[対対和:2,三暗刻:2]",);
        assert_eq!(yaku("55888m33j222s111p ?3j", &env), "[対対和:2,三暗刻:2]",);
        // 対対和にならない待ちがあるケース
        assert_eq!(yaku("33344455m22244s ?2m", &env), "[断么九:1,一盃口:1]");
        assert_eq!(
            yaku("33344455m22244s ?5m", &env),
            "[断么九:1,一盃口:1][断么九:1,対対和:2,三暗刻:2]",
        );
        assert_eq!(
            yaku("33344455m22244s ?4s", &env),
            "[断么九:1,対対和:2,三暗刻:2]",
        );
    }

    #[test]
    fn sananko() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 確定三暗刻の単騎待ち
        assert_eq!(yaku("111333s555m3459p ?9p", &env), "[三暗刻:2]");
        // 一手変わりで四暗刻になる両面待ち
        assert_eq!(yaku("111333s555m3499p ?2p", &env), "[三暗刻:2]");
        assert_eq!(yaku("111333s555m3499p ?5p", &env), "[三暗刻:2]");
        // ツモり四暗刻の双碰待ち
        assert_eq!(yaku("111333s555m3399p ?3p", &env), "[対対和:2,三暗刻:2]");
        assert_eq!(yaku("111333s555m3399p ?9p", &env), "[対対和:2,三暗刻:2]");
        // ツモり三暗刻の双碰待ち
        assert_eq!(
            yaku("111333s55m34599p !5m", &env),
            "[門前清自摸和:1,三暗刻:2]",
        );
        assert_eq!(yaku("111333s55m34599p ?5m", &env), "[]");
        // 注意を要する単騎待ち
        assert_eq!(yaku("111333s5556m345p ?6m", &env), "[三暗刻:2]");
        assert_eq!(yaku("111333s5556m345p ?4m", &env), "[]");
        assert_eq!(yaku("111333s5556m345p ?7m", &env), "[]");
        // 暗刻による面子の確定
        assert_eq!(yaku("111333s6m345p 5555m ?6m", &env), "[三暗刻:2]");
    }

    #[test]
    fn honro() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 副露したケース
        assert_eq!(
            yaku("11199p66j <777j ^111m ?9p", &env),
            "[役牌: 中:1,対対和:2,混老頭:2]",
        );
        // 門前のケース
        assert_eq!(
            yaku("11199p66j111m777j ?9p", &env),
            "[役牌: 中:1,対対和:2,三暗刻:2,混老頭:2]",
        );
        // 七対形のケース
        assert_eq!(yaku("66j1199p99s11m442j ?2j", &env), "[七対子:2,混老頭:2]",);
    }

    #[test]
    fn sandoko() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 対対和との複合/2副露したケース
        assert_eq!(
            yaku("222m7799s >222s >222p ?7s", &env),
            "[対対和:2,三色同刻:2]",
        );
        // 三暗刻との複合/門前のケース
        assert_eq!(
            yaku("333m33399p33789s !3s", &env),
            "[門前清自摸和:1,三暗刻:2,三色同刻:2]",
        );
    }

    #[test]
    fn sankantsu() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 対対和になっていないケース
        assert_eq!(
            yaku("11p45m 6666j ^888+8m ^2222p ?3m", &env),
            "[役牌: 發:1,三槓子:2]",
        );
        // 四槓子の一向聴
        assert_eq!(
            yaku("7774j >111+1j 9999s 2222m ?4j", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,役牌: 中:1,対対和:2,三暗刻:2,三槓子:2]",
        );
    }

    #[test]
    fn daburi() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.daburi = true;
        assert_eq!(yaku("1112345m345s123p ?6m", &env), "[平和:1,ダブル立直:2]");
    }

    #[test]
    fn shosan() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 高目大三元のケース
        assert_eq!(
            yaku("55577j567m22p >666j ?2p", &env),
            "[役牌: 白:1,役牌: 發:1,小三元:2]",
        );
        // 単騎待ちのケース
        assert_eq!(
            yaku("567m222p7j >666j <555j ?7j", &env),
            "[役牌: 白:1,役牌: 發:1,小三元:2]",
        );
    }

    #[test]
    fn honitsu() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 風牌の刻子があるケース
        assert_eq!(
            yaku("11456678m11166j ?6j", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,役牌: 發:1,混一色:3]",
        );
        // 一気通貫・役牌との複合
        assert_eq!(
            yaku("12345678m66644j ?3m", &env),
            "[役牌: 發:1,混一色:3][役牌: 發:1,混一色:3]",
        );
        assert_eq!(
            yaku("12345678m66644j ?6m", &env),
            "[役牌: 發:1,混一色:3][役牌: 發:1,混一色:3]",
        );
        assert_eq!(
            yaku("12345678m66644j ?9m", &env),
            "[役牌: 發:1,一気通貫:2,混一色:3]",
        );
        // 平和との複合
        assert_eq!(
            yaku("11223356678m44j ?4m", &env),
            "[平和:1,一盃口:1,混一色:3]",
        );
        assert_eq!(
            yaku("11223356678m44j ?7m", &env),
            "[平和:1,一盃口:1,混一色:3][一盃口:1,混一色:3]",
        );
        // 対対和・役牌との複合
        assert_eq!(
            yaku("11133355m44j ^666j ?5m", &env),
            "[役牌: 發:1,対対和:2,混一色:2]",
        );
        // 七対子との複合
        assert_eq!(yaku("11224488m14466j ?1j", &env), "[七対子:2,混一色:3]");
        // 多門張のケース
        assert_eq!(
            yaku("2223456777m111j ?1m", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,混一色:3]",
        );
        assert_eq!(
            yaku("2223456777m111j ?4m", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,混一色:3][役牌: 場風牌:1,役牌: 自風牌:1,混一色:3]",
        );
        assert_eq!(
            yaku("2223456777m111j ?7m", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,混一色:3][役牌: 場風牌:1,役牌: 自風牌:1,混一色:3]",
        );
        assert_eq!(
            yaku("2223456777m111j ?2m", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,混一色:3][役牌: 場風牌:1,役牌: 自風牌:1,混一色:3]",
        );
        assert_eq!(
            yaku("2223456777m111j ?5m", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,混一色:3][役牌: 場風牌:1,役牌: 自風牌:1,混一色:3]",
        );
        assert_eq!(
            yaku("2223456777m111j ?8m", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,混一色:3]",
        );
        assert_eq!(
            yaku("2223456777m111j ?3m", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,三暗刻:2,混一色:3]",
        );
        assert_eq!(
            yaku("2223456777m111j ?6m", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,三暗刻:2,混一色:3]",
        );
        // バカホンのケース
        assert_eq!(yaku("12334578m44j <789m ?6m", &env), "[混一色:2]");
        assert_eq!(yaku("12334578m44j <789m ?9m", &env), "[混一色:2]");
        // メンチン一向聴のケース
        assert_eq!(yaku("111234455667m3j ?3j", &env), "[混一色:3]");
    }

    #[test]
    fn junchan() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 純全帯么九のみのケース
        assert_eq!(yaku("11m123s789p789s99m ?1m", &env), "[純全帯么九:3]");
        assert_eq!(yaku("11m123s789p789s99m ?9m", &env), "[純全帯么九:3]");
        // 純全帯么九三色のケース
        assert_eq!(
            yaku("12m123s123p99s789m ?3m", &env),
            "[三色同順:2,純全帯么九:3]"
        );
        // 高目と安目の落差が激しいケース
        assert_eq!(
            yaku("123m123s23p99s789m ?1p", &env),
            "[平和:1,三色同順:2,純全帯么九:3]",
        );
        assert_eq!(yaku("123m123s23p99s789m ?4p", &env), "[平和:1]");
        // 片あがりのケース
        assert_eq!(yaku("123m999p99s78m <123s ?6m", &env), "[]");
        assert_eq!(yaku("123m999p99s78m <123s ?9m", &env), "[純全帯么九:2]");
        // 片あがりのケース2
        assert_eq!(
            yaku("11123m789p78999s ?1m", &env),
            "[純全帯么九:3][純全帯么九:3]",
        );
        assert_eq!(yaku("11123m789p78999s ?4m", &env), "[]");
        assert_eq!(yaku("11123m789p78999s ?6s", &env), "[]");
        assert_eq!(
            yaku("11123m789p78999s ?9s", &env),
            "[純全帯么九:3][純全帯么九:3]",
        );
        // 清老頭の一向聴であるケース
        assert_eq!(yaku("111m111p99s89m ^999p ?7m", &env), "[純全帯么九:2]");
        // 清一色と複合するケース
        assert_eq!(
            yaku("1122233378999p ?1p", &env),
            "[平和:1,一盃口:1,純全帯么九:3,清一色:6][清一色:6]",
        );
        assert_eq!(
            yaku("1122233378999p ?4p", &env),
            "[平和:1,一盃口:1,清一色:6]",
        );
        assert_eq!(yaku("1122233378999p ?6p", &env), "[三暗刻:2,清一色:6]");
        assert_eq!(
            yaku("1122233378999p ?9p", &env),
            "[三暗刻:2,清一色:6][清一色:6]",
        );
    }

    #[test]
    fn ryanpeko() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 平和との複合/高目で二盃口になるケース
        assert_eq!(yaku("334455m22334p99s ?1p", &env), "[平和:1,一盃口:1]");
        assert_eq!(
            yaku("334455m22334p99s ?4p", &env),
            "[平和:1,二盃口:3][七対子:2]",
        );
        // 4枚使いのあるケース
        assert_eq!(yaku("33445555m22344p ?3p", &env), "[断么九:1,二盃口:3]");
        // 同一順子4組あるケース
        assert_eq!(
            yaku("33334444555m22p ?2m", &env),
            "[断么九:1,平和:1,一盃口:1][断么九:1,三暗刻:2]"
        );
        assert_eq!(
            yaku("33334444555m22p ?5m", &env),
            "[断么九:1,平和:1,二盃口:3][断么九:1,三暗刻:2][断么九:1]"
        );
        assert_eq!(yaku("33334444555m22p ?2p", &env), "[断么九:1]");
    }

    #[test]
    fn kokushi() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 単騎待ち
        assert_eq!(yaku("19m19p19s1234577j ?6j", &env), "[国士無双:!1]");
        // 十三面待ち
        assert_eq!(
            yaku("19m19p19s1234567j ?1m", &env),
            "[国士無双十三面待ち:!2]",
        );
    }

    #[test]
    fn suanko() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 双碰待ちの場合
        assert_eq!(yaku("333s111s555m7799p !9p", &env), "[四暗刻:!1]");
        assert_eq!(yaku("333s111s555m7799p ?9p", &env), "[対対和:2,三暗刻:2]");
        assert_eq!(yaku("333s111s555m7799p !7p", &env), "[四暗刻:!1]");
        assert_eq!(yaku("333s111s555m7799p ?7p", &env), "[対対和:2,三暗刻:2]");
        // 注意を要する双碰待ち
        assert_eq!(
            yaku("666j111s2223344p ?2p", &env),
            "[一盃口:1,役牌: 發:1][一盃口:1,役牌: 發:1]",
        );
        assert_eq!(yaku("666j111s2223344p !3p", &env), "[四暗刻:!1]");
        assert_eq!(
            yaku("666j111s2223344p ?3p", &env),
            "[役牌: 發:1,対対和:2,三暗刻:2]",
        );
        assert_eq!(yaku("666j111s2223344p !4p", &env), "[四暗刻:!1]");
        assert_eq!(
            yaku("666j111s2223344p ?4p", &env),
            "[役牌: 發:1,対対和:2,三暗刻:2]",
        );
        assert_eq!(yaku("666j111s2223344p ?5p", &env), "[役牌: 發:1]");
        // 確定単騎
        assert_eq!(yaku("111j333555m333p5s ?5s", &env), "[四暗刻単騎:!2]");
        // 注意を要する単騎待ち
        assert_eq!(yaku("666888s222m1112p ?2p", &env), "[四暗刻単騎:!2]");
        assert_eq!(yaku("666888s222m1112p ?3p", &env), "[三暗刻:2]");
    }

    #[test]
    fn daisangen() {
        let env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        // 高目大三元のケース
        assert_eq!(yaku("11p456m55566777j ?6j", &env), "[大三元:!1]");
        assert_eq!(
            yaku("11p456m55566777j ?1p", &env),
            "[役牌: 白:1,役牌: 中:1,小三元:2]",
        );
        // 高目大三元のケース/三元牌を2副露しているケース
        assert_eq!(yaku("11p456m66j ^777j >555j ?6j", &env), "[大三元:!1]");
        assert_eq!(
            yaku("11p456m66j ^777j >555j ?1p", &env),
            "[役牌: 白:1,役牌: 中:1,小三元:2]",
        );
        // 大三元確定のケース
        assert_eq!(yaku("33s78p777j ^666j ^555j ?6p", &env), "[大三元:!1]");
        // 他の役満との複合
        assert_eq!(yaku("1155666777j999s !5j", &env), "[四暗刻:!1,大三元:!1]");
        assert_eq!(yaku("1155666777j999s ?5j", &env), "[大三元:!1]");
        assert_eq!(yaku("1155666777j999s !1j", &env), "[四暗刻:!1]");
        assert_eq!(
            yaku("1155666777j999s ?1j", &env),
            "[役牌: 發:1,役牌: 中:1,対対和:2,三暗刻:2,混老頭:2,小三元:2,混一色:3]",
        );
    }

    #[test]
    fn tsuiso() {
        let env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        // 副露した字一色
        assert_eq!(yaku("1112277j ^444j ^555j ?2j", &env), "[字一色:!1]");
        // 四喜和との複合
        assert_eq!(
            yaku("3334455j ^222j <111j ?4j", &env),
            "[字一色:!1,大四喜:!2]",
        );
        assert_eq!(
            yaku("3334455j ^222j <111j ?5j", &env),
            "[字一色:!1,小四喜:!1]",
        );
        // 字一色七対子
        assert_eq!(yaku("1122334455667j ?7j", &env), "[字一色:!1]");
    }

    #[test]
    fn shosushi() {
        let env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        // 単騎待ちのケース
        assert_eq!(yaku("1112224j234m <333j !4j", &env), "[小四喜:!1]");
        // 双碰待ちのケース
        assert_eq!(yaku("2244j234m <333j ^111j !2j", &env), "[小四喜:!1]");
        assert_eq!(yaku("2244j234m <333j ^111j !4j", &env), "[小四喜:!1]");
        // 両面待ちのケース
        assert_eq!(yaku("44j23m <333j >222j ^111j ?1m", &env), "[小四喜:!1]");
        assert_eq!(yaku("44j23m <333j >222j ^111j ?4m", &env), "[小四喜:!1]");
    }

    #[test]
    fn daisushi() {
        let env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        // 双碰待ちのケース
        assert_eq!(yaku("33344j22s >222j ^111j ?4j", &env), "[大四喜:!2]");
        assert_eq!(yaku("33344j22s >222j ^111j ?2s", &env), "[小四喜:!1]");
        // 単騎待ちのケース
        assert_eq!(
            yaku("3334445j >222j ^111j ?5j", &env),
            "[字一色:!1,大四喜:!2]",
        );
    }

    #[test]
    fn ryuiso() {
        let env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        // 緑一色が確定しているケース
        assert_eq!(yaku("22334466s66j >888s ?6s", &env), "[緑一色:!1]");
        assert_eq!(yaku("22334466s66j >888s ?6j", &env), "[緑一色:!1]");
        // 高目と安目があるケース
        assert_eq!(yaku("34666s66j <888s <423s ?2s", &env), "[緑一色:!1]");
        assert_eq!(yaku("34666s66j <888s <423s ?5s", &env), "[混一色:2]");
        // 変則待ちのケース
        assert_eq!(yaku("2233446888s <666j ?6s", &env), "[緑一色:!1]");
        assert_eq!(yaku("2233446888s <666j ?7s", &env), "[役牌: 發:1,混一色:2]");
        // 發のないケース
        assert_eq!(yaku("4466s <888s <333s ^222s ?4s", &env), "[緑一色:!1]");
        assert_eq!(yaku("4466s <888s <333s ^222s ?6s", &env), "[緑一色:!1]");
    }

    #[test]
    fn chinro() {
        let env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        // 副露したケース
        assert_eq!(yaku("11s111p11m <999s ^999m ?1s", &env), "[清老頭:!1]");
        assert_eq!(yaku("11s111p11m <999s ^999m ?1m", &env), "[清老頭:!1]");
        // 門前のケース/四暗刻との複合
        assert_eq!(
            yaku("111999s999m1119p ?9p", &env),
            "[四暗刻単騎:!2,清老頭:!1]",
        );
    }

    #[test]
    fn sukantsu() {
        let env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        assert_eq!(
            yaku("1m 5555j 3333j ^7777s ^8888p ?1m", &env),
            "[四槓子:!1]"
        );
    }

    #[test]
    fn churen() {
        let env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        assert_eq!(yaku("1113455678999p ?2p", &env), "[九蓮宝燈:!1]");
        assert_eq!(
            yaku("1112345678999m ?1m", &env),
            "[純正九蓮宝燈:!2][純正九蓮宝燈:!2]",
        );
        assert_eq!(yaku("1112345678999m ?2m", &env), "[純正九蓮宝燈:!2]");
        assert_eq!(
            yaku("1112345678999m ?3m", &env),
            "[純正九蓮宝燈:!2][純正九蓮宝燈:!2]",
        );
        assert_eq!(
            yaku("1112345678999m ?4m", &env),
            "[純正九蓮宝燈:!2][純正九蓮宝燈:!2]",
        );
        assert_eq!(yaku("1112345678999m ?5m", &env), "[純正九蓮宝燈:!2]");
        assert_eq!(
            yaku("1112345678999m ?6m", &env),
            "[純正九蓮宝燈:!2][純正九蓮宝燈:!2]",
        );
        assert_eq!(
            yaku("1112345678999m ?7m", &env),
            "[純正九蓮宝燈:!2][純正九蓮宝燈:!2]",
        );
        assert_eq!(yaku("1112345678999m ?8m", &env), "[純正九蓮宝燈:!2]");
        assert_eq!(
            yaku("1112345678999m ?9m", &env),
            "[純正九蓮宝燈:!2][純正九蓮宝燈:!2]",
        );
    }

    #[test]
    fn tenho() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.tenho = true;
        assert_eq!(yaku("1112345m345s123p ?6m", &env), "[天和:!1]");
    }

    #[test]
    fn chiho() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.chiho = true;
        assert_eq!(yaku("1112345m345s123p ?6m", &env), "[地和:!1]");
    }
}
