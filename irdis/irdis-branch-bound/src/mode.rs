pub(crate) mod sequential;

pub(crate) mod integrated;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub enum DeiceMode {
    Sequential,
    #[default]
    Integrated,
}
