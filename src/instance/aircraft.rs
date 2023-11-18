use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Aircraft {
    pub reg: Registration,
    pub model: Model,
    pub size_class: SizeClass,
}

impl Aircraft {
    pub fn new(reg: Registration, model: Model, size_class: SizeClass) -> Self {
        Self {
            reg,
            model,
            size_class,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Registration(String);

impl Registration {
    pub fn new(reg: impl Into<String>) -> Self {
        Self(reg.into())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Model(String);

impl Model {
    pub fn new(model: impl Into<String>) -> Self {
        Self(model.into())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SizeClass {
    Small,
    Medium,
    Large,
}
