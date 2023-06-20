use std::fmt::{Display, Debug};

pub struct Version {
    major: u8,
    minor: u8,
    patch: u32,
}

impl Version {
    const fn new(major: u8, minor: u8, path: u32) -> Self {
        Self { major, minor, patch: path }
    }

    pub fn to_string(&self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

pub static VERSION: Version = Version::new(1, 0, 0);
pub static NAME: &str = "rainbow16";