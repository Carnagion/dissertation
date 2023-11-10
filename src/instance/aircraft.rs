pub struct Aircraft {
    pub registration: Registration,
    pub model: Model,
    pub class: SizeClass,
}

pub struct Registration(String);

pub struct Model(String);

pub enum SizeClass {
    Medium,
    Large,
}
