use std::fmt::Display;

/// Crate semver struct with the `v1.25.8` format.
#[derive(Default, Debug, PartialEq)]
pub struct Version(Option<String>);

impl Version {
    /// Takes a string and prepends 'v' to it for formatting.
    pub fn from_string(mut s: String) -> Self {
        s.insert(0, 'v');
        Self(Some(s))
    }

    /// Checks if the version is of the `None` variant. Simply calls
    /// `Option::is_none`
    pub fn is_unset(&self) -> bool {
        self.0.is_none()
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &str {
        match &self.0 {
            Some(s) => &s,
            None => unreachable!()
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(v) => f.write_str(v),
            None => f.write_str("v0.0.0")
        }
    }
}