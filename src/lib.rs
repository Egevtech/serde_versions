use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Version {
        return Version {
            major,
            minor,
            patch,
        };
    }
}
