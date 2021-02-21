use crate::{
    agari_type::AgariType,
    hai::{self, Hai},
    hai_category::HaiCategory,
    hai_with_attr::HaiWithAttr,
    tacha::Tacha,
};
use enum_iterator::IntoEnumIterator;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Debug, Error)]
pub(crate) enum ErrorKind {
    #[error("number not found")]
    NumberNotFound,
    #[error("category not found at last")]
    CategoryNotFound,
    #[error("invalid char found: `{0}`")]
    InvalidChar(char),
    #[error("multiple prefix found: `{0}` and `{1}`")]
    MultiplePrefix(Prefix, Prefix),
    #[error("multiple number found: `{0}` and `{1}`")]
    MultipleNumber(u8, u8),
    #[error("multiple dora sign found: `$`")]
    MultipleDora,
    #[error("multiple category found: `{0}` and `{1}`")]
    MultipleCategory(HaiCategory, HaiCategory),
    #[error(transparent)]
    NewHai(#[from] hai::NewError),
}

impl From<Error> for ErrorKind {
    fn from(e: Error) -> Self {
        e.0
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Prefix {
    Tacha(Tacha),
    Kakan,
    Agari(AgariType),
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Tacha(tacha) => write!(f, "{}", tacha),
            Prefix::Kakan => write!(f, "+"),
            Prefix::Agari(agari) => write!(f, "{}", agari),
        }
    }
}
#[derive(Debug, Clone)]
pub(crate) struct HaiBuilder {
    allow_prefix: bool,
    prefix: Option<Prefix>,
    number: Option<u8>,
    category: Option<HaiCategory>,
    akadora: bool,
}

impl HaiBuilder {
    pub(crate) fn new(allow_prefix: bool) -> Self {
        Self {
            allow_prefix,
            prefix: None,
            number: None,
            category: None,
            akadora: false,
        }
    }

    fn set_prefix(&mut self, prefix: Prefix) -> Result<&mut Self, Error> {
        if let Some(old_prefix) = self.prefix {
            return Err(ErrorKind::MultiplePrefix(old_prefix, prefix).into());
        }
        self.prefix = Some(prefix);
        Ok(self)
    }

    fn set_number(&mut self, number: u8) -> Result<&mut Self, Error> {
        if let Some(old_number) = self.number {
            return Err(ErrorKind::MultipleNumber(old_number, number).into());
        }
        self.number = Some(number);
        Ok(self)
    }

    fn set_akadora(&mut self) -> Result<&mut Self, Error> {
        if self.akadora {
            return Err(ErrorKind::MultipleDora.into());
        }
        self.akadora = true;
        Ok(self)
    }

    pub(crate) fn category(&self) -> Option<HaiCategory> {
        self.category
    }

    pub(crate) fn set_category(&mut self, category: HaiCategory) -> Result<&mut Self, Error> {
        if let Some(old_category) = self.category {
            return Err(ErrorKind::MultipleCategory(old_category, category).into());
        }
        self.category = Some(category);
        Ok(self)
    }

    fn eat_prefix<'a>(&mut self, mut s: &'a str) -> Result<&'a str, Error> {
        while let Some((prefix, rest)) = parse_prefix(s) {
            self.set_prefix(prefix)?;
            s = rest;
        }
        Ok(s)
    }

    fn eat_number<'a>(&mut self, s: &'a str) -> Result<&'a str, Error> {
        let (number, rest) = parse_number(s)?;
        self.set_number(number)?;
        Ok(rest)
    }

    fn eat_akadora<'a>(&mut self, mut s: &'a str) -> Result<&'a str, Error> {
        while let Some(rest) = s.strip_prefix('$') {
            self.set_akadora()?;
            s = rest;
        }
        Ok(s)
    }

    fn eat_category<'a>(&mut self, mut s: &'a str) -> Result<&'a str, Error> {
        while let Some((category, rest)) = parse_category(s) {
            self.set_category(category)?;
            s = rest;
        }
        Ok(s)
    }

    pub(crate) fn eat_str<'a>(&mut self, mut s: &'a str) -> Result<&'a str, Error> {
        let original_s = s;
        if self.allow_prefix {
            s = self.eat_prefix(s)?;
        }
        s = self.eat_number(s)?;
        s = self.eat_akadora(s)?;
        s = self.eat_category(s)?;
        assert!(s != original_s);
        Ok(s)
    }

    pub(crate) fn eat_whole_str(&mut self, s: &str) -> Result<(), Error> {
        let rest = self.eat_str(s)?;
        if let Some(ch) = rest.chars().next() {
            return Err(ErrorKind::InvalidChar(ch).into());
        }
        Ok(())
    }

    pub(crate) fn build(self) -> Result<HaiWithAttr, Error> {
        use ErrorKind as E;
        use HaiWithAttr::*;
        let hai = match (self.number, self.category) {
            (Some(number), Some(category)) => {
                Hai::try_new(category, number, self.akadora).map_err(E::from)?
            }
            (None, _) => return Err(E::NumberNotFound.into()),
            (_, None) => return Err(E::CategoryNotFound.into()),
        };

        let res = match self.prefix {
            None => FromTehai(hai),
            Some(Prefix::Tacha(p)) => FromTacha(p, hai),
            Some(Prefix::Kakan) => Kakan(hai),
            Some(Prefix::Agari(agari)) => Agari(agari, hai),
        };
        Ok(res)
    }
}

fn parse_prefix(s: &str) -> Option<(Prefix, &str)> {
    for tacha in Tacha::into_enum_iter() {
        if let Some(rest) = s.strip_prefix(tacha.to_str()) {
            return Some((Prefix::Tacha(tacha), rest));
        }
    }
    if let Some(rest) = s.strip_prefix('+') {
        return Some((Prefix::Kakan, rest));
    }
    for agari in AgariType::into_enum_iter() {
        if let Some(rest) = s.strip_prefix(agari.to_str()) {
            return Some((Prefix::Agari(agari), rest));
        }
    }

    None
}

fn parse_category(s: &str) -> Option<(HaiCategory, &str)> {
    for c in HaiCategory::into_enum_iter() {
        if let Some(rest) = s.strip_prefix(c.to_str()) {
            return Some((c, rest));
        }
    }
    None
}

fn parse_number(s: &str) -> Result<(u8, &str), Error> {
    use ErrorKind as E;
    let mut chars = s.chars();
    let ch = chars.next().ok_or(E::NumberNotFound)?;
    let number = ch.to_digit(10).ok_or(E::InvalidChar(ch))?;
    Ok((number as u8, chars.as_str()))
}
