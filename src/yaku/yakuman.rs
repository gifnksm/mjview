use super::common;
use crate::{agari::Agari, env::Env, hai_category::HaiCategory, mentsu::MentsuKind};

pub(super) fn kokushi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_toitsu() == 1 && agari.num_single() == 12).then(|| {
        if let MentsuKind::Toitsu(..) = agari.machi_mentsu().kind() {
            ("国士無双十三面待ち", 2)
        } else {
            ("国士無双", 1)
        }
    })
}

pub(super) fn suanko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_anko() + agari.num_ankan() == 4).then(|| {
        if let MentsuKind::Toitsu(..) = agari.machi_mentsu().kind() {
            ("四暗刻単騎", 2)
        } else {
            ("四暗刻", 1)
        }
    })
}

pub(super) fn daisangen(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (common::sangenpai_bits(agari) == 0b111).then(|| ("大三元", 1))
}

pub(super) fn tsuiso(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_jihai() == agari.num_hai()).then(|| ("字一色", 1))
}

pub(super) fn shosushi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    let janto = agari.janto()?.head();
    (janto.is_kazehai() && common::kazehai_bits(agari) ^ (1 << (janto.number() - 1)) == 0b1111)
        .then(|| ("小四喜", 1))
}

pub(super) fn daisushi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (common::kazehai_bits(agari) == 0b1111).then(|| ("大四喜", 2))
}

pub(super) fn ryuiso(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
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

pub(super) fn chinro(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_jihai() == 0 && agari.num_yaochuhai() == agari.num_hai()).then(|| ("清老頭", 1))
}

pub(super) fn sukantsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_ankan() + agari.num_minkan() == 4).then(|| ("四槓子", 1))
}

pub(super) fn churen(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
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

pub(super) fn tenho(_agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    env.tenho.then(|| {
        if env.jikaze == env.bakaze {
            ("天和", 1)
        } else {
            ("地和", 1)
        }
    })
}

#[cfg(test)]
mod test {
    use super::{super::common::test::yaku, *};
    use crate::hai::Hai;
    use std::str::FromStr;

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

        // index.html のサンプル
        assert_eq!(
            yaku("19m19p19s1234567j !1j", &env),
            "[国士無双十三面待ち:!2]",
        )
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

        // 青天井時、三暗刻、対対和とは複合しない
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.aotenjo = true;
        assert_eq!(yaku("333s111s555m7999p ?7p", &env), "[四暗刻単騎:!2]");
        // 四暗刻単騎と門前清自摸和は複合する
        assert_eq!(
            yaku("333s111s555m7999p !7p", &env),
            "[四暗刻単騎:!2,門前清自摸和:1]"
        );
        // ツモり四暗刻と門前清自摸和は複合しない
        assert_eq!(yaku("333s111s555m7799p !9p", &env), "[四暗刻:!1]");
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

        let mut env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        env.aotenjo = true;
        // 青天井時、混全帯么九とは複合しない
        assert_eq!(
            yaku("1112277j ^444j ^555j ?2j", &env),
            "[字一色:!1,役牌: 場風牌:1,役牌: 自風牌:1,役牌: 白:1,対対和:2]"
        );
        // 青天井時、七対子と複合するassert_eq!(
        assert_eq!(yaku("1122334455667j ?7j", &env), "[字一色:!1,七対子:2]");
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

        let mut env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        env.aotenjo = true;
        // 青天井時、役牌、混一色と複合する
        assert_eq!(
            yaku("1112224j234m <333j !4j", &env),
            "[小四喜:!1,役牌: 場風牌:1,役牌: 自風牌:1,混一色:2]",
        );
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

        let mut env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        env.aotenjo = true;
        // 青天井時、役牌、対対和とは複合しない
        assert_eq!(
            yaku("33344j22s >222j ^111j ?4j", &env),
            "[大四喜:!2,混一色:2]",
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

        let mut env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        env.aotenjo = true;
        // 發無し緑一色を認めているため、青天井時は混一色と複合する
        assert_eq!(yaku("22334466s66j >888s ?6s", &env), "[緑一色:!1,混一色:2]");
        // 發無し緑一色を認めているため、青天井時は清一色、断么九と複合する
        assert_eq!(
            yaku("4466s <888s <333s ^222s ?4s", &env),
            "[緑一色:!1,断么九:1,対対和:2,清一色:5]",
        );
        // 役牌: 發は複合する
        assert_eq!(
            yaku("2233446888s <666j ?6s", &env),
            "[緑一色:!1,役牌: 發:1,混一色:2]",
        );
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

        let mut env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        env.aotenjo = true;
        // 青天井時、純全帯么九とは複合しない。七対子の4枚使いも認めていないため、対対和とも複合しない
        assert_eq!(yaku("11s111p11m <999s ^999m ?1s", &env), "[清老頭:!1]");
    }

    #[test]
    fn sukantsu() {
        let env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        assert_eq!(
            yaku("1m 5555j 3333j ^7777s ^8888p ?1m", &env),
            "[四槓子:!1]",
        );

        // index.html のサンプル
        assert_eq!(
            yaku("1m 9999p ^333+3s >4444j 6666m !1m", &env),
            "[四槓子:!1]",
        );

        // 青天井時、三槓子、対対和とは複合しない
        let mut env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        env.aotenjo = true;
        assert_eq!(
            yaku("1m 5555j 3333j ^7777s ^8888p ?1m", &env),
            "[四槓子:!1,役牌: 白:1]",
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

        let mut env = Env::new_empty(Hai::from_str("2j").unwrap(), Hai::from_str("2j").unwrap());
        env.aotenjo = true;
        // 青天井時、清一色とは複合しない
        assert_eq!(yaku("1113455678999p ?2p", &env), "[九蓮宝燈:!1]");
        // 一気通貫とは複合する
        assert_eq!(yaku("1111345678999p ?2p", &env), "[九蓮宝燈:!1,一気通貫:2]");
    }

    #[test]
    fn tenho() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.tenho = true;
        assert_eq!(yaku("1112345m345s123p !6m", &env), "[天和:!1]");
    }

    #[test]
    fn chiho() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("2j").unwrap());
        env.tenho = true;
        assert_eq!(yaku("1112345m345s123p !6m", &env), "[地和:!1]");
    }
}
