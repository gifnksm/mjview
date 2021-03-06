use super::common;
use crate::{agari::Agari, env::Env};

pub(super) fn chinitsu(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    if super::yakuman::churen(agari, env).is_some() {
        return None;
    }
    (agari.num_manzu() == agari.num_hai()
        || agari.num_pinzu() == agari.num_hai()
        || agari.num_souzu() == agari.num_hai())
    .then(|| ("清一色", common::kuisagari(agari, 6)))
}

#[cfg(test)]
mod test {
    use super::{super::common::test::yaku, *};
    use crate::hai::Hai;
    use std::str::FromStr;

    #[test]
    fn chinitsu() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        // 多面張のケース
        assert_eq!(yaku("1113456677778m ?2m", &env), "[清一色:6][清一色:6]");
        assert_eq!(yaku("1113456677778m ?5m", &env), "[清一色:6][清一色:6]");
        assert_eq!(yaku("1113456677778m ?8m", &env), "[一盃口:1,清一色:6]");
        assert_eq!(yaku("1113456677778m ?3m", &env), "[清一色:6]");
        assert_eq!(yaku("1113456677778m ?6m", &env), "[清一色:6][清一色:6]");
        assert_eq!(yaku("1113456677778m ?9m", &env), "[清一色:6]");
        // 高目と安目があるケース
        assert_eq!(
            yaku("1223345556789m ?1m", &env),
            "[平和:1,一盃口:1,一気通貫:2,清一色:6]",
        );
        assert_eq!(
            yaku("1223345556789m ?4m", &env),
            "[平和:1,一気通貫:2,清一色:6][平和:1,一気通貫:2,清一色:6]",
        );
        assert_eq!(
            yaku("1223345556789m ?7m", &env),
            "[平和:1,清一色:6][清一色:6]",
        );
        assert_eq!(yaku("1223345556789m ?6m", &env), "[清一色:6]");
        assert_eq!(yaku("1223345556789m ?9m", &env), "[清一色:6]");
        // 副露したケース
        assert_eq!(yaku("2344555m <789m <213m ?1m", &env), "[清一色:5]");
        assert_eq!(
            yaku("2344555m <789m <213m ?4m", &env),
            "[清一色:5][清一色:5]",
        );
        assert_eq!(
            yaku("2344555m <789m <213m ?3m", &env),
            "[清一色:5][清一色:5]",
        );
        assert_eq!(
            yaku("2344555m <789m <213m ?6m", &env),
            "[一気通貫:1,清一色:5]",
        );
        // 特殊なメンチンの例
        assert_eq!(
            yaku("1112346678999s ?6s", &env),
            "[清一色:6][清一色:6][清一色:6]",
        );
        assert_eq!(yaku("1112346678999s ?9s", &env), "[清一色:6][清一色:6]");
        assert_eq!(yaku("1112346678999s ?5s", &env), "[九蓮宝燈:!1]");
        // 特殊なメンチンの例2
        assert_eq!(
            yaku("2233445667788p ?5p", &env),
            "[断么九:1,平和:1,二盃口:3,清一色:6]\
            [断么九:1,二盃口:3,清一色:6]\
            [断么九:1,平和:1,二盃口:3,清一色:6]\
            [断么九:1,七対子:2,清一色:6]",
        );
        assert_eq!(
            yaku("2233445667788p ?2p", &env),
            "[断么九:1,平和:1,一盃口:1,清一色:6]\
            [断么九:1,一盃口:1,清一色:6]",
        );
        assert_eq!(
            yaku("2233445667788p ?8p", &env),
            "[断么九:1,平和:1,一盃口:1,清一色:6]\
            [断么九:1,一盃口:1,清一色:6]",
        );
        // 特殊なメンチンの例3
        assert_eq!(yaku("2233444666888s ?1s", &env), "[清一色:6]");
        assert_eq!(yaku("2233444666888s ?2s", &env), "[緑一色:!1]");
        assert_eq!(yaku("2233444666888s ?3s", &env), "[緑一色:!1]");
        assert_eq!(yaku("2233444666888s ?4s", &env), "[緑一色:!1][緑一色:!1]");
        assert_eq!(
            yaku("2233444666888s ?5s", &env),
            "[断么九:1,一盃口:1,清一色:6]",
        );
    }
}
