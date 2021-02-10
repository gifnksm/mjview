use crate::{hai::Hai, hai_category::HaiCategory, mentsu::Mentsu};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShuntsuCandidates<'a> {
    used_bits: u16,
    hai: &'a [Hai],
}

impl<'a> ShuntsuCandidates<'a> {
    pub(crate) fn new(hai: &'a [Hai], mut used_bits: u16) -> Self {
        assert!(hai.len() <= 14);
        assert!(used_bits < (0b1 << hai.len()));
        debug_assert!(hai.windows(2).all(|w| w[0] <= w[1])); // assert sorted

        used_bits |= !((0b1 << hai.len()) - 1); // 範囲外をマスク;
        Self { used_bits, hai }
    }
}

impl<'a> Iterator for ShuntsuCandidates<'a> {
    type Item = (Mentsu, u16);

    fn next(&mut self) -> Option<Self::Item> {
        fn count_same(hai: &[Hai], idx: usize) -> usize {
            1 + hai[idx + 1..]
                .iter()
                .filter(|h| hai[idx].is_same(h))
                .count()
        }
        fn next_unused(used_bits: u16, start: usize) -> usize {
            start + (used_bits >> start).trailing_ones() as usize
        }
        loop {
            let start = self.used_bits.trailing_ones() as usize;
            if start + 2 >= self.hai.len() {
                break;
            }
            let start_hai = self.hai[start];
            if start_hai.category() == HaiCategory::Jihai {
                break;
            }
            let start_count = count_same(self.hai, start);

            let mid = next_unused(self.used_bits, start + start_count);
            if mid + 1 >= self.hai.len() {
                break;
            }
            let mid_hai = self.hai[mid];
            if !mid_hai.is_next_to(&start_hai) {
                self.used_bits |= ((0b1 << start_count) - 1) << start;
                continue;
            }
            let mid_count = count_same(self.hai, mid);

            let end = next_unused(self.used_bits, mid + mid_count);
            if end >= self.hai.len() {
                break;
            }
            let end_hai = self.hai[end];
            if !end_hai.is_next_to(&mid_hai) {
                self.used_bits |= ((0b1 << (start_count + mid_count)) - 1) << start;
                continue;
            }

            let use_bits = (0b1 << start) | (0b1 << mid) | (0b1 << end);
            self.used_bits |= use_bits;
            return Some((Mentsu::shuntsu([start_hai, mid_hai, end_hai]), use_bits));
        }
        self.used_bits = u16::MAX;
        None
    }
}

#[cfg(test)]
mod test {
    use crate::jun_tehai::JunTehai;
    use std::str::FromStr;

    use super::ShuntsuCandidates;

    #[test]
    fn shuntsu() {
        fn test(input: &str, used_bits: u16, expected: &[(&str, u16)]) {
            let hai_vec = JunTehai::from_str(input).unwrap();
            let toitsu = ShuntsuCandidates::new(hai_vec.as_slice(), used_bits)
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

        test("123p", 0, &[("123p", 0b111)]);
        test("111m111p111s111j", 0, &[]);
        test(
            "111122223333m",
            0,
            &[
                ("123m", 0b0001_0001_0001),
                ("123m", 0b0010_0010_0010),
                ("123m", 0b0100_0100_0100),
                ("123m", 0b1000_1000_1000),
            ],
        );
        test(
            "11112222334455m",
            0,
            &[("123m", 0b0001_0001_0001), ("123m", 0b0010_0010_0010)],
        );
        test("", 0, &[]);
        test("1p", 0, &[]);
        test("12p", 0, &[]);
        test("123j", 0, &[]);
        test("1123345p55s", 0, &[("123p", 0b1101), ("345p", 0b1110000)]);
        test(
            "111122223333p",
            0b0001_0010_0100,
            &[
                ("123p", 0b0010_0001_0001),
                ("123p", 0b0100_0100_0010),
                ("123p", 0b1000_1000_1000),
            ],
        );
    }
}
