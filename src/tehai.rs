use crate::{agari::Agari, agari_hai::AgariHai, furo::Furo, jun_tehai::JunTehai};
use std::{fmt, str::FromStr};
use thiserror::Error;

/// 手牌
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Tehai {
    jun_tehai: JunTehai,
    furo: Vec<Furo>,
    agari: AgariHai,
}

impl fmt::Display for Tehai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.jun_tehai)?;
        for furo in &self.furo {
            write!(f, " {}", furo)?;
        }
        write!(f, " {}", self.agari)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub(crate) enum ParseError {
    #[error("no jun-tehai found")]
    NoJunTehai,
    #[error("no agari-hai found")]
    NoAgariHai,
    #[error("invalid hai count: `{0}`")]
    InvalidHaiCount(usize),
    #[error("jun-tehai: {0}")]
    JunTehai(#[from] <JunTehai as FromStr>::Err),
    #[error("furo: {0}")]
    Furo(#[from] <Furo as FromStr>::Err),
    #[error("agari-hai: {0}")]
    AgariHai(#[from] <AgariHai as FromStr>::Err),
}

impl FromStr for Tehai {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseError as E;
        let mut chunks = s.split_whitespace().peekable();

        let jun_tehai = JunTehai::from_str(chunks.next().ok_or(E::NoJunTehai)?)?;

        let mut furo = vec![];
        while let Some(chunk) = chunks.peek() {
            if Agari::is_agari_str(chunk) {
                break;
            }
            let chunk = chunks.next().unwrap();
            furo.push(Furo::from_str(chunk)?);
        }

        let agari = AgariHai::from_str(chunks.next().ok_or(E::NoAgariHai)?)?;

        let hai_count = jun_tehai.as_vec().len() + furo.len() * 3 + 1;
        if hai_count != 14 {
            return Err(E::InvalidHaiCount(hai_count));
        }

        Ok(Self {
            jun_tehai,
            furo,
            agari,
        })
    }
}
