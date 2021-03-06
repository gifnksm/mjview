use crate::{agari::Agari, hai::Hai, hai_category::HaiCategory, mentsu::MentsuKind};

pub(super) fn kuisagari(agari: &Agari, fan: u32) -> u32 {
    if agari.tehai().is_menzen() {
        fan
    } else {
        fan - 1
    }
}

pub(super) fn sangenpai_bits(agari: &Agari) -> u8 {
    let mut bits = 0;
    for mentsu in agari.all_mentsu().filter(|mentsu| {
        matches!(
            mentsu.kind(),
            MentsuKind::Kotsu(..) | MentsuKind::Kantsu(..)
        )
    }) {
        let head = mentsu.head();
        if head.category() != HaiCategory::Jihai {
            continue;
        }
        let bit = match head.number() {
            5 => 0b001,
            6 => 0b010,
            7 => 0b100,
            _ => continue,
        };
        bits |= bit;
    }
    bits
}

pub(super) fn kazehai_bits(agari: &Agari) -> u8 {
    let mut bits = 0;
    for mentsu in agari.all_mentsu().filter(|mentsu| {
        matches!(
            mentsu.kind(),
            MentsuKind::Kotsu(..) | MentsuKind::Kantsu(..)
        )
    }) {
        let head = mentsu.head();
        if head.category() != HaiCategory::Jihai {
            continue;
        }
        let bit = match head.number() {
            1 => 0b0001,
            2 => 0b0010,
            3 => 0b0100,
            4 => 0b1000,
            _ => continue,
        };
        bits |= bit;
    }
    bits
}

pub(super) fn num_peko(agari: &Agari) -> usize {
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

#[cfg(test)]
pub(super) mod test {
    use super::super::RankKind;
    use crate::{env::Env, tehai::Tehai};
    use std::str::FromStr;

    pub(in super::super) fn yaku(s: &str, env: &Env) -> String {
        let tehai = Tehai::from_str(s).unwrap();
        let comb = tehai.to_agari_combinations();
        comb.into_iter()
            .map(|agari| {
                let s = super::super::judge(&agari, env)
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
}
