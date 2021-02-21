use crate::{hai::Hai, machi::Machi, mentsu::Mentsu};
use std::iter::Enumerate;

#[derive(Debug, Clone)]
pub(crate) struct MachiCombinations<I>
where
    I: IntoIterator,
{
    mentsu: Enumerate<I::IntoIter>,
    agari: Hai,
    last: Option<Mentsu>,
}

impl<I> MachiCombinations<I>
where
    I: IntoIterator<Item = Mentsu>,
{
    pub(crate) fn new(mentsu: I, agari: Hai) -> Self {
        let mentsu = mentsu.into_iter().enumerate();
        Self {
            mentsu,
            agari,
            last: None,
        }
    }
}

impl<I> Iterator for MachiCombinations<I>
where
    I: IntoIterator<Item = Mentsu>,
{
    type Item = (usize, Machi);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, mentsu)) = self.mentsu.next() {
            if self.last == Some(mentsu) {
                continue;
            }
            self.last = Some(mentsu);
            if let Some(machi) = mentsu.to_machi(self.agari) {
                return Some((idx, machi));
            }
        }
        None
    }
}
