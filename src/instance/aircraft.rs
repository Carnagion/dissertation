#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Aircraft {
    pub registration: Registration,
    pub model: Model,
    pub class: SizeClass,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Registration(String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Model(String);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SizeClass {
    Medium,
    Large,
}
