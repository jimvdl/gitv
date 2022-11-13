use std::{ fmt::Display, borrow::Cow};

pub struct Hash<'a>(Cow<'a, str>);

impl<'a> Hash<'a> {
    pub fn from_string(s: String) -> Self {
        Self(Cow::Owned(s))
    }

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