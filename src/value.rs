use crate::Index;
use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
};
use core::{cmp, fmt};
use ecow::EcoString;

#[derive(Clone)]
pub enum Value {
    Num(f64),
    String(EcoString),
    Bool(bool),
}

impl Value {
    #[must_use]
    pub fn to_bool(&self) -> bool {
        match self {
            Self::Num(num) => *num != 0.0 && !num.is_nan(),
            Self::String(s) => {
                !s.is_empty() && s != "0" && !s.eq_ignore_ascii_case("false")
            }
            Self::Bool(b) => *b,
        }
    }

    #[must_use]
    pub fn try_to_num(&self) -> Option<f64> {
        match self {
            Self::Num(num) if num.is_nan() => None,
            Self::Num(num) => Some(*num),
            Self::String(s) => try_str_to_num(s),
            Self::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        }
    }

    #[must_use]
    pub fn to_num(&self) -> f64 {
        self.try_to_num().unwrap_or(0.0)
    }

    #[must_use]
    pub fn to_cow_str(&self) -> Cow<str> {
        match self {
            Self::Num(num) => Cow::Owned(number_to_string(*num)),
            Self::String(s) => Cow::Borrowed(s),
            Self::Bool(true) => Cow::Borrowed("true"),
            Self::Bool(false) => Cow::Borrowed("false"),
        }
    }

    #[must_use]
    pub fn to_index(&self) -> Option<Index> {
        // TODO: Handle "all", "random" and "any"
        match self {
            Self::String(s) if s == "last" => Some(Index::Last),
            _ => self
                .try_to_num()
                .and_then(|n| (n as usize).checked_sub(1).map(Index::Nth)),
        }
    }

    #[must_use]
    pub fn compare(&self, other: &Self) -> cmp::Ordering {
        if let (Some(lhsn), Some(rhsn)) =
            (self.try_to_num(), other.try_to_num())
        {
            // Comparing floats only fails when NaN is involved, which
            // `try_to_num()` filters out.
            lhsn.partial_cmp(&rhsn).unwrap()
        } else {
            // TODO: Do this without allocating new strings
            self.to_cow_str()
                .to_lowercase()
                .cmp(&other.to_cow_str().to_lowercase())
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::String(EcoString::default())
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(num) => fmt::Debug::fmt(num, f),
            Self::String(s) => fmt::Debug::fmt(s, f),
            Self::Bool(b) => fmt::Debug::fmt(b, f),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_cow_str())
    }
}

fn try_str_to_num(s: &str) -> Option<f64> {
    match s.trim() {
        "Infinity" | "+Infinity" => Some(f64::INFINITY),
        "-Infinity" => Some(f64::NEG_INFINITY),
        "inf" | "+inf" | "-inf" => None,
        s => s.parse().ok().filter(|n: &f64| !n.is_nan()),
    }
}

fn number_to_string(num: f64) -> String {
    ryu_js::Buffer::new().format(num).to_owned()
}
