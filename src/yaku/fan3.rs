use super::common;
use crate::{agari::Agari, env::Env, mentsu::MentsuKind};

pub(super) fn honitsu(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.num_jihai() > 0
        && agari.num_jihai() < agari.num_hai()
        && ((agari.num_jihai() + agari.num_manzu() == agari.num_hai())
            || (agari.num_jihai() + agari.num_souzu() == agari.num_hai())
            || (agari.num_jihai() + agari.num_pinzu() == agari.num_hai())))
    .then(|| ("混一色", common::kuisagari(agari, 3)))
}

pub(super) fn junchan(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari
        .all_mentsu()
        .any(|mentsu| matches!(mentsu.kind(), MentsuKind::Shuntsu(..)))
        && agari.num_jihai() == 0
        && agari
            .all_mentsu()
            .all(|mentsu| mentsu.iter().any(|hai| hai.is_yaochuhai())))
    .then(|| ("純全帯么九", common::kuisagari(agari, 3)))
}

pub(super) fn ryanpeko(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    (agari.tehai().is_menzen() && common::num_peko(agari) == 2).then(|| ("二盃口", 3))
}

#[cfg(test)]
mod test {
    use super::{super::common::test::yaku, *};
    use crate::hai::Hai;
    use std::str::FromStr;

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
}
