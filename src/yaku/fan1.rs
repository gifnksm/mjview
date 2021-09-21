use super::common;
use crate::{
    agari::Agari,
    agari_type::AgariType,
    env::{Env, RichiType},
    hai::Hai,
    hai_category::HaiCategory,
    machi::Machi,
    mentsu::MentsuKind,
};

pub(super) fn richi(_agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    (env.richi == Some(RichiType::Richi)).then(|| ("立直", 1))
}

pub(super) fn ippatsu(_agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.ippatsu.then(|| ("一発", 1))
}

pub(super) fn tsumo(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    // 天和/地和、ツモり四暗刻とは複合しない
    (!env.tenho
        && (agari.num_anko() + agari.num_ankan() != 4
            || matches!(agari.machi_mentsu().kind(), MentsuKind::Toitsu(..)))
        && agari.tehai().is_menzen()
        && agari.tehai().agari_hai().type_() == AgariType::Tsumo)
        .then(|| ("門前清自摸和", 1))
}

pub(super) fn tanyao(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    agari
        .tehai()
        .all_hai()
        .all(|hai| !hai.is_yaochuhai())
        .then(|| ("断么九", 1))
}

pub(super) fn pinfu(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    (agari.tehai().is_menzen()
        && agari.machi() == Machi::Ryanmen
        && agari.num_anshun() + agari.num_minshun() == 4
        && agari.janto()?.compute_fu(true, env) == 0)
        .then(|| ("平和", 1))
}

pub(super) fn ipeko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.tehai().is_menzen() && common::num_peko(agari) == 1).then(|| ("一盃口", 1))
}

pub(super) fn bakaze(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    if common::kazehai_bits(agari) == 0b1111 {
        return None;
    }
    let count = yakuhai(agari, |hai| env.bakaze.is_same(hai));
    (count > 0).then(|| ("役牌: 場風牌", count as u32))
}

pub(super) fn jikaze(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    if common::kazehai_bits(agari) == 0b1111 {
        return None;
    }
    let count = yakuhai(agari, |hai| env.jikaze.is_same(hai));
    (count > 0).then(|| ("役牌: 自風牌", count as u32))
}

pub(super) fn haku(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    if common::sangenpai_bits(agari) == 0b111 {
        return None;
    }
    let count = yakuhai(agari, |hai| {
        hai.number() == 5 && hai.category() == HaiCategory::Jihai
    });
    (count > 0).then(|| ("役牌: 白", count as u32))
}

pub(super) fn hatsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    if common::sangenpai_bits(agari) == 0b111 {
        return None;
    }
    let count = yakuhai(agari, |hai| {
        hai.number() == 6 && hai.category() == HaiCategory::Jihai
    });
    (count > 0).then(|| ("役牌: 發", count as u32))
}

pub(super) fn chun(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    if common::sangenpai_bits(agari) == 0b111 {
        return None;
    }
    let count = yakuhai(agari, |hai| {
        hai.number() == 7 && hai.category() == HaiCategory::Jihai
    });
    (count > 0).then(|| ("役牌: 中", count as u32))
}

pub(super) fn rinshan(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.rinshan.then(|| {
        if agari.tehai().agari_hai().type_() == AgariType::Tsumo {
            ("嶺上開花", 1)
        } else {
            ("搶槓", 1)
        }
    })
}

pub(super) fn haitei(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.haitei.then(|| {
        if agari.tehai().agari_hai().type_() == AgariType::Tsumo {
            ("海底撈月", 1)
        } else {
            ("河底撈魚", 1)
        }
    })
}

fn yakuhai(agari: &Agari, f: impl Fn(&Hai) -> bool) -> usize {
    agari
        .all_mentsu()
        .filter(|mentsu| {
            matches!(
                mentsu.kind(),
                MentsuKind::Kotsu(..) | MentsuKind::Kantsu(..)
            )
        })
        .map(|mentsu| mentsu.head())
        .filter(f)
        .count()
}

#[cfg(test)]
mod test {
    use super::{super::common::test::yaku, *};
    use std::str::FromStr;

    #[test]
    fn rich() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.richi = Some(RichiType::Richi);
        assert_eq!(yaku("344556m24678s66j ?3s", &env), "[立直:1]");
    }

    #[test]
    fn ippatsu() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.richi = Some(RichiType::Richi);
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

        // 青天常時、天和/地和とは複合しない
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.tenho = true;
        env.aotenjo = true;
        assert_eq!(yaku("345567m1113368s !7s", &env), "[天和:!1]");
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("2j").unwrap());
        env.tenho = true;
        env.aotenjo = true;
        assert_eq!(yaku("345567m1113368s !7s", &env), "[地和:!1]");
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

        // 青天井の場合大三元とは複合しない
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.aotenjo = true;
        assert_eq!(yaku("33s78p777j ^666j ^555j ?6p", &env), "[大三元:!1]");
    }
}
