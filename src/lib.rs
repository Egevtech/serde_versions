use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
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

impl std::string::ToString for Version {
    fn to_string(&self) -> String {
        return format!("{}.{}.{}", self.major, self.minor, self.patch);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat(usize),
    NumberParseError(std::num::ParseIntError),
}

impl std::str::FromStr for Version {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 3 {
            return Err(ParseError::InvalidFormat(parts.len()));
        }

        return Ok(Version {
            major: parts[0]
                .parse::<u32>()
                .map_err(ParseError::NumberParseError)?,
            minor: parts[1]
                .parse::<u32>()
                .map_err(ParseError::NumberParseError)?,
            patch: parts[2]
                .parse::<u32>()
                .map_err(ParseError::NumberParseError)?,
        });
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

#[cfg(test)]
mod tests {
    use super::Version;
    use serde::Deserialize;
    use serde_json;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        version: Version,
    }

    #[test]
    fn test() {
        let json_str = r#"
            {
                "version": {
                    "major": 1,
                    "minor": 0,
                    "patch": 0
                }
            }
            "#;

        let dt: Data = match serde_json::from_str(json_str) {
            Ok(dt) => dt,
            Err(err) => panic!("Failed to unwrap json: {}", err),
        };

        assert_eq!(
            dt,
            Data {
                version: Version::new(1, 0, 0)
            }
        )
    }

    #[test]
    fn ord_test() {
        let ver1: Version = Version::new(1, 0, 0);
        let ver2: Version = Version::new(1, 0, 1);
        let ver3: Version = Version::new(2, 3, 1);
        let ver4: Version = Version::new(0, 0, 0);

        /*
         * ver1 < ver2
         * ver2 < ver3
         * ver3 > ver4
         * ver1 > ver4
         */

        assert!(ver1 < ver2);
        assert!(ver2 < ver3);
        assert!(ver3 > ver4);
        assert!(ver1 > ver4);
    }
}
