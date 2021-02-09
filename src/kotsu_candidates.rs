use crate::{hai::Hai, mentsu::Mentsu};

#[derive(Debug, Clone, Copy)]
pub(crate) struct KotsuCandidates<'a> {
    visited_bits: u16,
    hai: &'a [Hai],
}

impl<'a> KotsuCandidates<'a> {
    pub(crate) fn new(hai: &'a [Hai], used_bits: u16) -> Self {
        assert!(hai.len() <= 14);
        assert!(used_bits < (0b1 << hai.len()));
        debug_assert!(hai.windows(2).all(|w| w[0] <= w[1])); // assert sorted

        let used_bits = used_bits | !((0b1 << hai.len()) - 1); // 範囲外をマスク
        let visited_bits = used_bits | (used_bits >> 1) | (used_bits >> 2); // 番兵
        Self { visited_bits, hai }
    }
}

impl<'a> Iterator for KotsuCandidates<'a> {
    type Item = (Mentsu, u16);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let start = self.visited_bits.trailing_ones() as usize;
            if start == 16 {
                break;
            }
            let start_hai = self.hai[start];
            let mid_hai = self.hai[start + 1];
            let end_hai = self.hai[start + 2];
            if !start_hai.is_same(&mid_hai) {
                self.visited_bits |= 0b1 << start;
                continue;
            }
            if !start_hai.is_same(&end_hai) {
                self.visited_bits |= 0b11 << start;
                continue;
            }
            let use_bits = 0b111 << start;
            let mut skip_bits = 0;
            // Skip same hai
            for skip_idx in start + 3..self.hai.len() {
                let skip_hai = self.hai[skip_idx];
                if !start_hai.is_same(&skip_hai) {
                    break;
                }
                skip_bits |= 0b1 << skip_idx;
            }
            self.visited_bits |= use_bits | skip_bits;
            return Some((Mentsu::kotsu([start_hai, mid_hai, end_hai]), use_bits));
        }
        assert_eq!(self.visited_bits, u16::MAX);
        None
    }
}

#[cfg(test)]
mod test {
    use crate::jun_tehai::JunTehai;
    use std::str::FromStr;

    use super::KotsuCandidates;

    #[test]
    fn kotsu() {
        fn test(input: &str, used_bits: u16, expected: &[(&str, u16)]) {
            let hai_vec = JunTehai::from_str(input).unwrap();
            let toitsu = KotsuCandidates::new(hai_vec.as_slice(), used_bits)
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

        test(
            "111m111p111s111j",
            0,
            &[
                ("111m", 0b111),
                ("111p", 0b111 << 3),
                ("111s", 0b111 << 6),
                ("111j", 0b111 << 9),
            ],
        );
        test(
            "111122223333m",
            0,
            &[("111m", 0b111), ("222m", 0b111 << 4), ("333m", 0b111 << 8)],
        );
        test("", 0, &[]);
        test("1p", 0, &[]);
        test("1123345p55s", 0, &[]);
        test("111222333p", 0b001 | (0b100 << 3) | (0b010 << 6), &[]);
        test("11112222p", 0b0001 | (0b0010 << 4), &[("111p", 0b1110)]);
        test(
            "33334444p",
            0b0100 | (0b1000 << 4),
            &[("444p", 0b0111 << 4)],
        );
    }
}
