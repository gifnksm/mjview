use crate::{
    hai::Hai, hai_category::HaiCategory, kotsu_candidates::KotsuCandidates, mentsu::Mentsu,
    shuntsu_candidates::ShuntsuCandidates, toitsu_candidates::ToitsuCandidates,
};

pub(crate) fn combinations(hai: &[Hai]) -> Vec<Vec<Mentsu>> {
    if let Some(comb) = to_kokushi(hai) {
        return vec![comb]; // 他の形とは複合しないため return して良い
    }

    let mut result = vec![];

    // 七対子
    if hai.len() == 14
        && (0..7).all(|i| hai[i * 2].is_same(&hai[i * 2 + 1]))
        && (1..7).all(|i| !hai[i * 2 - 1].is_same(&hai[i * 2]))
    {
        let comb = (0..7)
            .map(|i| Mentsu::toitsu([hai[2 * i], hai[2 * i + 1]]))
            .collect::<Vec<_>>();
        result.push(comb);
    }

    for toitsu in ToitsuCandidates::new(hai, 0, true) {
        let all_kotsu = KotsuCandidates::new(hai, toitsu.1).collect::<Vec<_>>();
        for kotsu_bit in 0..(1 << all_kotsu.len()) {
            let mut comb = vec![toitsu.0];
            let mut used_bits = toitsu.1;
            for (i, kotsu) in all_kotsu.iter().enumerate() {
                if kotsu_bit & (1 << i) != 0 {
                    comb.push(kotsu.0);
                    used_bits |= kotsu.1;
                }
            }
            for shuntsu in ShuntsuCandidates::new(hai, used_bits) {
                used_bits |= shuntsu.1;
                comb.push(shuntsu.0);
            }
            if used_bits + 1 == 1 << hai.len() {
                comb.sort();
                result.push(comb);
            }
        }
    }
    result.sort();
    result
}

fn to_kokushi(hai: &[Hai]) -> Option<Vec<Mentsu>> {
    if hai.len() != 14 {
        return None;
    }
    #[derive(Clone, Copy)]
    enum Slot {
        Empty,
        One(Hai),
        Two(Hai, Hai),
    }
    let mut hai_vec = vec![Slot::Empty; 13];
    let mut cnt = 0;
    for h in hai {
        use HaiCategory::*;
        let idx = match (h.category(), h.number()) {
            (Manzu, 1) => 0,
            (Manzu, 9) => 1,
            (Pinzu, 1) => 2,
            (Pinzu, 9) => 3,
            (Souzu, 1) => 4,
            (Souzu, 9) => 5,
            (Jihai, n) => 5 + (n as usize),
            _ => return None,
        };
        hai_vec[idx] = match hai_vec[idx] {
            Slot::Empty => {
                cnt += 1;
                Slot::One(*h)
            }
            Slot::One(same_hai) => Slot::Two(same_hai, *h),
            Slot::Two(_, _) => return None,
        }
    }
    if cnt != 13 {
        return None;
    }
    let mut tehai_mentsu = hai_vec
        .into_iter()
        .map(|slot| match slot {
            Slot::Empty => unreachable!(),
            Slot::One(h0) => Mentsu::single([h0]),
            Slot::Two(h0, h1) => Mentsu::toitsu([h0, h1]),
        })
        .collect::<Vec<_>>();
    tehai_mentsu.sort();
    Some(tehai_mentsu)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::jun_tehai::JunTehai;
    use std::str::FromStr;

    #[test]
    fn comb() {
        fn parse(input: &str) -> Vec<Vec<Mentsu>> {
            let hai_vec = JunTehai::from_str(input).unwrap();
            combinations(hai_vec.as_slice())
        }
        fn test(input: &str, expected: &[&[&str]]) {
            let comb = parse(input);
            let left = comb
                .iter()
                .map(|comb| {
                    comb.iter()
                        .map(|mentsu| mentsu.to_string())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let right = expected
                .iter()
                .map(|comb| {
                    comb.iter()
                        .map(|mentsu| mentsu.to_string())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            assert_eq!(left, right);
        }

        test("11122345p", &[&["345p", "111p", "22p"]]);
        test(
            "11122233344455m",
            &[
                &["123m", "123m", "123m", "444m", "55m"],
                &["234m", "234m", "234m", "111m", "55m"],
                &["234m", "345m", "345m", "111m", "22m"],
                &["111m", "222m", "333m", "444m", "55m"],
            ],
        );
        test(
            "11223344556677p",
            &[
                &["123p", "123p", "456p", "456p", "77p"],
                &["123p", "123p", "567p", "567p", "44p"],
                &["234p", "234p", "567p", "567p", "11p"],
                &["11p", "22p", "33p", "44p", "55p", "66p", "77p"],
            ],
        );

        // 九連宝燈
        test(
            "11112345678999m",
            &[&["123m", "456m", "789m", "111m", "99m"]],
        );
        test(
            "11122345678999m",
            &[&["345m", "678m", "111m", "999m", "22m"]],
        );
        test(
            "11123345678999m",
            &[&["123m", "345m", "678m", "999m", "11m"]],
        );
        test(
            "11123445678999m",
            &[&["234m", "456m", "789m", "111m", "99m"]],
        );
        test(
            "11123455678999m",
            &[&["234m", "678m", "111m", "999m", "55m"]],
        );
        test(
            "11123456678999m",
            &[&["123m", "456m", "678m", "999m", "11m"]],
        );
        test(
            "11123456778999m",
            &[&["234m", "567m", "789m", "111m", "99m"]],
        );
        test(
            "11123456788999m",
            &[&["234m", "567m", "111m", "999m", "88m"]],
        );
        test(
            "11123456789999m",
            &[&["123m", "456m", "789m", "999m", "11m"]],
        );

        // 天地創造 :-)
        test(
            "55555555555555j",
            &[&["555j", "555j", "555j", "555j", "55j"]],
        );

        for input in DATA {
            let comb = parse(input);
            if comb.is_empty() {
                panic!("failed: {}", input);
            }
        }
    }

    // test data from https://www.engineer-log.com/entry/2018/06/14/mahjong-algorithm
    const DATA: &[&str] = &[
        "23333444556688m",
        "22333456667788m",
        "22344445556677m",
        "11123334445577m",
        "22223333444556m",
        "11222345556677m",
        "22333344555667m",
        "11333445566678m",
        "11122223334455m",
        "22555566677788m",
        "23333444555566m",
        "22566667778899m",
        "22444567778899m",
        "22444455666778m",
        "12222333445599m",
        "22223344455688m",
        "11123334445555m",
        "33344555566678m",
        "44455667778999m",
        "11112233344566m",
        "11444556667778m",
        "11225566778899m",
        "44445555666778m",
        "12222333445588m",
        "22234555667777m",
        "33345666778888m",
        "11122334445666m",
        "22223334445588m",
        "33345666777788m",
        "11122334445677m",
        "22233345556677m",
        "11223344667799m",
        "11123444555566m",
        "44455567778899m",
        "33444455666778m",
        "22234445556666m",
        "11222334455567m",
        "44456667778888m",
        "11123344455688m",
        "11222334445556m",
        "11444566777889m",
        "11123334445588m",
        "11333344555667m",
        "22234555666677m",
        "11122333444566m",
        "44566667778899m",
        "55666677788899m",
        "33334455566799m",
        "11555667778889m",
        "11333455566677m",
        "22223344455699m",
        "33344445556677m",
        "33555566777889m",
        "22233445556799m",
        "11122333444588m",
        "11122223344456m",
        "22223334445599m",
        "34444555666677m",
        "44445566677899m",
        "55556666777889m",
        "22444556677789m",
        "11122333444599m",
        "11112223334499m",
        "11334455667799m",
        "33345566677899m",
        "11123344455666m",
        "33334445556699m",
        "33444566777889m",
        "11122233444556m",
        "11666677788899m",
        "33344555666788m",
        "22233334445556m",
        "11123334445566m",
        "11566667778899m",
        "11224466778899m",
        "11224455667799m",
        "22444556667778m",
        "12222333444455m",
        "22234445556677m",
        "33444455566677m",
        "22333344455566m",
        "11123334445599m",
        "33444556677789m",
        "11122333444577m",
        "11112223334466m",
        "11122223334445m",
        "22234455566667m",
        "22223334445577m",
        "11223355668899m",
        "11444455666778m",
        "11123444556688m",
        "44555667778889m",
        "11122334445699m",
        "11333456667788m",
        "11112223334488m",
        "55566667778889m",
        "11233334445566m",
        "11334455668899m",
        "33345556667799m",
        "22233344555667m",
        "34444555667799m",
        "11223344557799m",
        "11224455667788m",
        "22333445556667m",
        "22234445556688m",
        "22234444555667m",
        "11224455668899m",
        "22234555667799m",
        "11112233344599m",
        "33344445556667m",
        "44445566677888m",
        "11112223334477m",
        "55556677788999m",
        "11112233344588m",
        "11112222333445m",
        "22234455566799m",
        "11123444556699m",
        "33555667778889m",
        "22333445566678m",
        "33566667778899m",
        "12222333445577m",
        "22444566777889m",
        "22233444455567m",
        "44455666677789m",
        "22555677788899m",
        "44455556677789m",
        "44555566777889m",
        "22233445556788m",
        "11224455778899m",
        "44455566777889m",
        "33444567778899m",
        "11444566677788m",
        "44456667778899m",
        "22335566778899m",
        "33334444555667m",
        "11223344668899m",
        "22234455566777m",
        "44456777888899m",
        "33344556667899m",
        "44455556667778m",
        "11223344557788m",
        "33666677788899m",
        "11112233344555m",
        "55567778889999m",
        "11444455566677m",
        "11455556667788m",
        "33345556667788m",
        "33344456667788m",
        "22233444555699m",
        "44555566677788m",
        "11222233444556m",
        "11122333344456m",
        "11344445556677m",
        "11222344555667m",
        "44445556667799m",
        "33555677788899m",
        "22233444555677m",
        "11123344455699m",
        "11333445556667m",
        "44456677788889m",
        "22333455666778m",
        "33455556667788m",
        "11123344455556m",
        "11334466778899m",
        "33555566677788m",
        "11444556677789m",
        "44456677788999m",
        "11122234445566m",
        "22555667778889m",
        "22455556667788m",
        "33444556667778m",
        "22233445556777m",
        "33344445566678m",
        "11555566677788m",
        "33344555666799m",
        "22555566777889m",
        "33345566677778m",
        "33345556667777m",
        "33334455566788m",
        "22233334455567m",
        "22234445556699m",
        "33334445556688m",
        "11333344455566m",
        "44455556667788m",
        "33345566677888m",
        "11123333444556m",
        "33344556667888m",
        "11222344455566m",
        "33345555666778m",
        "22234455566788m",
        "22333455566677m",
        "44455666777899m",
        "23333444556699m",
        "11333455666778m",
        "11223344558899m",
        "11444567778899m",
        "11335566778899m",
        "33334455566777m",
        "45555666777788m",
        "44456666777889m",
        "11123344455677m",
        "33444566677788m",
        "11123444556666m",
        "22444455566677m",
        "22223344455666m",
        "22233444555688m",
        "11222233344455m",
        "44456777889999m",
        "44555677788899m",
        "22444566677788m",
        "22666677788899m",
        "22233334445566m",
        "44666677788899m",
        "11122334445688m",
        "22334455668899m",
        "33344455666778m",
        "56666777888899m",
        "11555566777889m",
        "55566667778899m",
        "11112233344577m",
        "22223344455677m",
        "11555677788899m",
    ];
}
