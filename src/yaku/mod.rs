use crate::{
    agari::Agari,
    agari_type::AgariType,
    env::Env,
    rank::{Rank, RankKind},
};
use js_sys::Array;
use num_bigint::BigUint;
use std::{borrow::Cow, cmp::Ordering, iter};
use wasm_bindgen::prelude::*;

mod common;
mod dora;
mod fan1;
mod fan2;
mod fan3;
mod fan6;
mod yakuman;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Yaku {
    name: Cow<'static, str>,
    point: BigUint,
    fu: u32,
    rank: Rank,
    detail: Vec<(&'static str, Rank)>,
}

impl PartialEq for Yaku {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Yaku {}

impl PartialOrd for Yaku {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Yaku {
    fn cmp(&self, other: &Self) -> Ordering {
        self.point
            .cmp(&other.point)
            .then_with(|| self.rank.cmp(&other.rank))
            .then_with(|| self.fu.cmp(&other.fu))
    }
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
    pub fn point_js(&self) -> String {
        self.point.to_string()
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

    #[wasm_bindgen(js_name = compare)]
    pub fn compare_js(&self, other: &Yaku) -> i32 {
        match self.cmp(other) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

fn judge(agari: &Agari, env: &Env) -> (Rank, Vec<(&'static str, Rank)>) {
    let mut total_fan = 0;
    let mut list = vec![];

    // 役満
    if let Some((yakuman_count, mut yakuman_list)) =
        judge_list(agari, env, &YAKUMAN_LIST, Rank::new_yakuman)
    {
        if !env.aotenjo {
            return (Rank::new_yakuman(yakuman_count), yakuman_list);
        }
        total_fan += yakuman_count * 13; // 役満は13飜扱い
        list.append(&mut yakuman_list);
    }

    // 通常の役
    if let Some((yaku_fan, mut yaku_list)) = judge_list(agari, env, &YAKU_LIST, Rank::new_fan) {
        total_fan += yaku_fan;
        list.append(&mut yaku_list);
    }

    // ドラ
    if !list.is_empty() {
        // 他の役がある場合のみカウント
        if let Some((dora_fan, mut dora_list)) = judge_list(agari, env, &DORA_LIST, Rank::new_fan) {
            total_fan += dora_fan;
            list.append(&mut dora_list);
        }
    }

    (Rank::new_fan(total_fan), list)
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

fn compute_point(agari: &Agari, env: &Env, fu: u32, rank: Rank) -> (Cow<'static, str>, BigUint) {
    let (name, base_point) = compute_base_point(env, fu, rank);
    let is_oya = env.jikaze.number() == 1;
    let is_ron = agari.tehai().agari_hai().type_() == AgariType::Ron;
    fn round(point: BigUint) -> BigUint {
        (point + 99u32) / 100u32 * 100u32
    }
    let point = match (is_oya, is_ron) {
        (true, true) => round(base_point * 6u32),
        (false, true) => round(base_point * 4u32),
        (true, false) => round(base_point * 2u32) * 3u32,
        (false, false) => round(&base_point * 2u32) + round(base_point) * 2u32,
    };

    (name, point)
}

fn compute_base_point(env: &Env, fu: u32, rank: Rank) -> (Cow<'static, str>, BigUint) {
    if env.aotenjo {
        match rank.kind() {
            RankKind::Fan(0) => return ("無役".into(), 0u32.into()),
            RankKind::Fan(fan) => return ("".into(), fu * BigUint::from(2u32).pow(fan + 2)),
            RankKind::Yakuman(_) => unreachable!(),
        }
    }

    let (name, point) = match rank.kind() {
        RankKind::Fan(0) => ("無役".into(), 0),
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
        RankKind::Yakuman(n) => (format!("{}倍役満", n).into(), (n * 8000)),
    };
    (name, BigUint::from(point))
}

type JudgeFn = fn(agari: &Agari, env: &Env) -> Option<(&'static str, u32)>;

const YAKU_LIST: &[JudgeFn] = &[
    fan1::richi,
    fan1::ippatsu,
    fan1::tsumo,
    fan1::tanyao,
    fan1::pinfu,
    fan1::ipeko,
    fan1::bakaze,
    fan1::jikaze,
    fan1::haku,
    fan1::hatsu,
    fan1::chun,
    fan1::rinshan,
    fan1::haitei,
    fan2::sanshoku,
    fan2::ittsu,
    fan2::chanta,
    fan2::chitoi,
    fan2::toitoi,
    fan2::sananko,
    fan2::honro,
    fan2::sandoko,
    fan2::sankantsu,
    fan2::shosan,
    fan2::daburi,
    fan3::honitsu,
    fan3::junchan,
    fan3::ryanpeko,
    fan6::chinitsu,
];

const YAKUMAN_LIST: &[JudgeFn] = &[
    yakuman::kokushi,
    yakuman::suanko,
    yakuman::daisangen,
    yakuman::tsuiso,
    yakuman::shosushi,
    yakuman::daisushi,
    yakuman::ryuiso,
    yakuman::chinro,
    yakuman::sukantsu,
    yakuman::churen,
    yakuman::tenho,
];

const DORA_LIST: &[JudgeFn] = &[dora::dora, dora::uradora, dora::akadora];
