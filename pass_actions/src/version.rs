use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u32, u32, u32);

impl Version {
    pub fn major(&self) -> u32 {
        self.0
    }

    pub fn minor(&self) -> u32 {
        self.1
    }

    pub fn patch(&self) -> u32 {
        self.2
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VersionParseError;

impl Display for VersionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VersionParseError")
    }
}

impl std::error::Error for VersionParseError {}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split('.');
        let mut next_version = move || {
            tokens
                .next()
                .and_then(|it| it.parse().ok())
                .ok_or(VersionParseError)
        };
        Ok(Version(next_version()?, next_version()?, next_version()?))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}
