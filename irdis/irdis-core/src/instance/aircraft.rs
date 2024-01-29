use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Aircraft {
    pub registration: Registration,
    pub model: Model,
    pub size_class: SizeClass,
}

impl Aircraft {
    pub fn new(registration: Registration, model: Model, size_class: SizeClass) -> Self {
        Self {
            registration,
            model,
            size_class,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Registration(pub String);

impl Registration {
    pub fn new<R>(reg: R) -> Self
    where
        R: Into<String>,
    {
        Self(reg.into())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Model(pub String);

impl Model {
    pub fn new<M>(model: M) -> Self
    where
        M: Into<String>,
    {
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
