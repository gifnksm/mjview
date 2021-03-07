use super::common;
use crate::{
    agari::Agari,
    env::{Env, RichiType},
    hai_category::HaiCategory,
    mentsu::MentsuKind,
};

pub(super) fn sanshoku(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
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
        .then(|| ("三色同順", common::kuisagari(agari, 2)))
}

pub(super) fn ittsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
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
    (manzu == 0b111 || souzu == 0b111 || pinzu == 0b111)
        .then(|| ("一気通貫", common::kuisagari(agari, 2)))
}

pub(super) fn chanta(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari
        .all_mentsu()
        .any(|mentsu| matches!(mentsu.kind(), MentsuKind::Shuntsu(..)))
        && agari.num_jihai() > 0
        && agari
            .all_mentsu()
            .all(|mentsu| mentsu.iter().any(|hai| hai.is_yaochuhai())))
    .then(|| ("混全帯么九", common::kuisagari(agari, 2)))
}

pub(super) fn chitoi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_toitsu() == 7).then(|| ("七対子", 2))
}

pub(super) fn toitoi(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    // 四暗刻、四槓子、清老頭、大四喜とは複合しない
    (agari.num_anko() + agari.num_minko() + agari.num_ankan() + agari.num_minkan() == 4
        && agari.num_ankan() + agari.num_minkan() < 4
        && agari.num_anko() < 4
        && agari.num_yaochuhai() - agari.num_jihai() < agari.num_hai()
        && common::kazehai_bits(agari) != 0b1111)
        .then(|| ("対対和", 2))
}

pub(super) fn sananko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_anko() + agari.num_ankan() == 3).then(|| ("三暗刻", 2))
}

pub(super) fn honro(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_single() == 0 // 国士無双とは複合しない
        && agari.num_jihai() > 0
        && agari.num_jihai() < agari.num_hai()
        && agari.tehai().all_hai().all(|hai| hai.is_yaochuhai()))
    .then(|| ("混老頭", 2))
}

pub(super) fn sandoko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
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

pub(super) fn sankantsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_ankan() + agari.num_minkan() == 3).then(|| ("三槓子", 2))
}

pub(super) fn shosan(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    let janto = agari.janto()?.head();
    (janto.is_sangenpai() && common::sangenpai_bits(agari) ^ (1 << (janto.number() - 5)) == 0b111)
        .then(|| ("小三元", 2))
}

pub(super) fn daburi(_agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    (env.richi == Some(RichiType::Daburi)).then(|| ("ダブル立直", 2))
}

#[cfg(test)]
mod test {
    use super::{super::common::test::yaku, *};
    use crate::hai::Hai;
    use std::str::FromStr;

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

        // index.html のサンプル
        assert_eq!(yaku("12344m123p33j <213s ?3j", &env), "[三色同順:1]")
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

        // 青天常時、国士無双とは複合しない
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.aotenjo = true;
        assert_eq!(yaku("19m19s19p1123456j ?7j", &env), "[国士無双:!1]");
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
    fn daburi() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        env.richi = Some(RichiType::Daburi);
        assert_eq!(yaku("1112345m345s123p ?6m", &env), "[平和:1,ダブル立直:2]");
    }
}
