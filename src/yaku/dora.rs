use crate::{agari::Agari, env::Env};

pub(super) fn dora(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    let count = agari
        .tehai()
        .all_hai()
        .map(|hai| env.dora.iter().filter(|dora| hai.is_next_to(dora)).count())
        .sum::<usize>();
    (count > 0).then(|| ("ドラ", count as u32))
}

pub(super) fn uradora(agari: &Agari, env: &Env) -> Option<(&'static str, u32)> {
    let _ = env.richi?;
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

pub(super) fn akadora(agari: &Agari, _env: &Env) -> Option<(&'static str, u32)> {
    let count = agari.tehai().all_hai().filter(|hai| hai.akadora()).count();
    (count > 0).then(|| ("赤ドラ", count as u32))
}

#[cfg(test)]
mod test {
    use super::{super::common::test::yaku, *};
    use crate::{env::RichiType, hai::Hai};
    use std::str::FromStr;

    #[test]
    fn dora() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());

        env.dora = vec![Hai::from_str("1m").unwrap(), Hai::from_str("5m").unwrap()];
        assert_eq!(yaku("1112345m345s123p ?6m", &env), "[平和:1,ドラ:2]");

        // 無役の場合はドラはつかない
        assert_eq!(yaku("1145m345s123p <123m ?6m", &env), "[]");

        // ドラ表時牌が 9m, 9p, 9s の時は 1m, 1p, 1s がドラになる
        env.dora = vec![
            Hai::from_str("9m").unwrap(),
            Hai::from_str("9p").unwrap(),
            Hai::from_str("9s").unwrap(),
        ];
        assert_eq!(yaku("111789m123p11s99s ?9s", &env), "[純全帯么九:3,ドラ:6]");

        // ドラ表時牌が 4j, 7j の時は 1j, 5j がドラになる
        env.dora = vec![Hai::from_str("4j").unwrap()];
        assert_eq!(
            yaku("234m567s456p1115j ?5j", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,ドラ:3]"
        );
        env.dora = vec![Hai::from_str("7j").unwrap()];
        assert_eq!(
            yaku("234m567s456p1115j ?5j", &env),
            "[役牌: 場風牌:1,役牌: 自風牌:1,ドラ:2]"
        );
    }

    #[test]
    fn uradora() {
        let mut env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());

        env.uradora = vec![Hai::from_str("1m").unwrap(), Hai::from_str("5m").unwrap()];
        env.richi = Some(RichiType::Richi);
        assert_eq!(
            yaku("1112345m345s123p ?6m", &env),
            "[立直:1,平和:1,裏ドラ:2]"
        );

        // 立直していない場合は裏ドラはつかない
        env.richi = None;
        assert_eq!(yaku("1112345m345s123p ?6m", &env), "[平和:1]");

        // 無役の場合はドラはつかない
        assert_eq!(yaku("1145m345s123p <123m ?6m", &env), "[]");
    }

    #[test]
    fn akadora() {
        let env = Env::new_empty(Hai::from_str("1j").unwrap(), Hai::from_str("1j").unwrap());
        assert_eq!(yaku("1112345$m345$s123p ?6m", &env), "[平和:1,赤ドラ:2]");
    }
}
