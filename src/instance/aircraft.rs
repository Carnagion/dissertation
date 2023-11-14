use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Aircraft {
    pub registration: Registration,
    pub model: Model,
    pub class: SizeClass,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Registration(String);

impl Registration {
    pub fn new(registration: impl Into<String>) -> Self {
        Self(registration.into())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Model(String);

impl Model {
    pub fn new(model: impl Into<String>) -> Self {
        Self(model.into())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SizeClass {
    Medium,
    Large,
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("invalid size class")]
pub struct ParseClassError;

impl FromStr for SizeClass {
    type Err = ParseClassError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "M" => Ok(Self::Medium),
            "L" => Ok(Self::Large),
            _ => Err(ParseClassError),
        }
    }
}
