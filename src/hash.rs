use std::{ fmt::Display, borrow::Cow};

/// A git commit hash wrapper. Not to be confused with [`std::hash::Hash`].
#[derive(Debug)]
pub struct Hash<'a>(Cow<'a, str>);

impl<'a> Hash<'a> {
    /// Takes an owned string and simply wraps it with `Cow::Owned`.
    pub fn from_string(s: String) -> Self {
        Self(Cow::Owned(s))
    }

    /// Borrows `s` for as long as Self lives.
    pub fn from_borrow(s: Cow<'a, str>) -> Self {
        Self(s)
    }
}

impl<'a> AsRef<str> for Hash<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> PartialEq<&Hash<'a>> for &str {
    fn eq(&self, other: &&Hash) -> bool {
        **self == *other.0
    }
}

impl<'a> Display for Hash<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}