use crate::{hai::Hai, mentsu::Mentsu};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ToitsuCandidates<'a> {
    visited_bits: u16,
    dedup: bool,
    hai: &'a [Hai],
}

impl<'a> ToitsuCandidates<'a> {
    pub(crate) fn new(hai: &'a [Hai], used_bits: u16, dedup: bool) -> Self {
        assert!(hai.len() <= 14);
        assert!(used_bits < (0b1 << hai.len()));
        debug_assert!(hai.windows(2).all(|w| w[0] <= w[1])); // assert sorted

        let used_bits = used_bits | !((0b1 << hai.len()) - 1); // 範囲外をマスク
        let visited_bits = used_bits | (used_bits >> 1); // 番兵
        Self {
            visited_bits,
            dedup,
            hai,
        }
    }
}

impl<'a> Iterator for ToitsuCandidates<'a> {
    type Item = (Mentsu, u16);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let start = self.visited_bits.trailing_ones() as usize;
            if start == 16 {
                break;
            }
            let start_hai = self.hai[start];
            let next_hai = self.hai[start + 1];
            if !start_hai.is_same(&next_hai) {
                self.visited_bits |= 0b1 << start;
                continue;
            }
            let use_bits = 0b11 << start;
            let mut skip_bits = 0;
            if self.dedup {
                // Skip same hai
                for skip_idx in start + 2..self.hai.len() {
                    let skip_hai = self.hai[skip_idx];
                    if !start_hai.is_same(&skip_hai) {
                        break;
                    }
                    skip_bits |= 0b1 << skip_idx;
                }
            }
            self.visited_bits |= use_bits | skip_bits;
            return Some((Mentsu::toitsu([start_hai, next_hai]), use_bits));
        }
        assert_eq!(self.visited_bits, u16::MAX);
        None
    }
}

#[cfg(test)]
mod test {
    use crate::jun_tehai::JunTehai;
    use std::str::FromStr;

    use super::ToitsuCandidates;

    #[test]
    fn toitsu() {
        fn test0(input: &str, used_bits: u16, dedup: bool, expected: &[(&str, u16)]) {
            let hai_vec = JunTehai::from_str(input).unwrap();
            let toitsu = ToitsuCandidates::new(hai_vec.as_slice(), used_bits, dedup)
                .map(|(mentsu, bits)| (mentsu.to_string(), bits))
                .collect::<Vec<_>>();
            assert_eq!(
                expected
                    .iter()
                    .map(|(mentsu, bits)| (mentsu.to_string(), *bits))
                    .collect::<Vec<_>>(),
                toitsu
            );
        }
        fn test(
            input: &str,
            used_bits: u16,
            dedup_expected: &[(&str, u16)],
            dup_expected: Option<&[(&str, u16)]>,
        ) {
            test0(input, used_bits, true, dedup_expected);
            let dup_expected = dup_expected.unwrap_or(dedup_expected);
            test0(input, used_bits, false, dup_expected);
        }

        test(
            "1123345p55s",
            0,
            &[("11p", 0b11), ("33p", 0b11 << 3), ("55s", 0b11 << 7)],
            None,
        );
        test(
            "111m111p111s111j",
            0,
            &[
                ("11m", 0b11),
                ("11p", 0b11 << 3),
                ("11s", 0b11 << 6),
                ("11j", 0b11 << 9),
            ],
            None,
        );
        test(
            "111122223333m",
            0,
            &[("11m", 0b11), ("22m", 0b11 << 4), ("33m", 0b11 << 8)],
            Some(&[
                ("11m", 0b11),
                ("11m", 0b11 << 2),
                ("22m", 0b11 << 4),
                ("22m", 0b11 << 6),
                ("33m", 0b11 << 8),
                ("33m", 0b11 << 10),
            ]),
        );
        test("", 0, &[], None);
        test("1p", 0, &[], None);
        test(
            "111222333p",
            0b001 | (0b100 << 3) | (0b010 << 6),
            &[("11p", 0b110), ("22p", 0b011 << 3)],
            None,
        );
    }
}
